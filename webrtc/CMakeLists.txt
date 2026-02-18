cmake_minimum_required(VERSION 4.2)

# Only interpret if() arguments as variables or keywords when unquoted.
cmake_policy(SET CMP0054 NEW)
# MSVC runtime library flags are selected by an abstraction.
cmake_policy(SET CMP0091 NEW)

# deps.json を読み込む（project() の前）
file(READ "${CMAKE_CURRENT_SOURCE_DIR}/deps.json" DEPS_JSON)
string(JSON WEBRTC_BUILD_VERSION GET ${DEPS_JSON} "webrtc_build_version")
string(JSON WEBRTC_BASE_URL GET ${DEPS_JSON} "webrtc_base_url")

# execute_process の実行結果を共通で検査する
function(execute_process_checked)
  set(options)
  set(oneValueArgs WORKING_DIRECTORY ERROR_MESSAGE)
  set(multiValueArgs COMMAND)
  cmake_parse_arguments(EXEC "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

  if(NOT EXEC_COMMAND)
    message(FATAL_ERROR "execute_process_checked requires COMMAND")
  endif()

  if(EXEC_WORKING_DIRECTORY)
    execute_process(
      COMMAND ${EXEC_COMMAND}
      WORKING_DIRECTORY "${EXEC_WORKING_DIRECTORY}"
      RESULT_VARIABLE EXEC_RESULT
      OUTPUT_VARIABLE EXEC_STDOUT
      ERROR_VARIABLE EXEC_STDERR
      OUTPUT_STRIP_TRAILING_WHITESPACE
      ERROR_STRIP_TRAILING_WHITESPACE
    )
  else()
    execute_process(
      COMMAND ${EXEC_COMMAND}
      RESULT_VARIABLE EXEC_RESULT
      OUTPUT_VARIABLE EXEC_STDOUT
      ERROR_VARIABLE EXEC_STDERR
      OUTPUT_STRIP_TRAILING_WHITESPACE
      ERROR_STRIP_TRAILING_WHITESPACE
    )
  endif()

  if(NOT EXEC_RESULT EQUAL 0)
    if(EXEC_ERROR_MESSAGE)
      set(ERROR_PREFIX "${EXEC_ERROR_MESSAGE}")
    else()
      set(ERROR_PREFIX "Command failed")
    endif()
    message(FATAL_ERROR
      "${ERROR_PREFIX}: ${EXEC_COMMAND}\nstdout: ${EXEC_STDOUT}\nstderr: ${EXEC_STDERR}")
  endif()
endfunction()

# shallow clone を共通化する
function(git_shallow_clone)
  set(options)
  set(oneValueArgs WORKING_DIRECTORY REPOSITORY COMMIT NAME)
  cmake_parse_arguments(SHALLOW "${options}" "${oneValueArgs}" "" ${ARGN})

  if(NOT SHALLOW_WORKING_DIRECTORY OR NOT SHALLOW_REPOSITORY OR NOT SHALLOW_COMMIT OR NOT SHALLOW_NAME)
    message(FATAL_ERROR "git_shallow_clone requires WORKING_DIRECTORY REPOSITORY COMMIT NAME")
  endif()

  file(MAKE_DIRECTORY "${SHALLOW_WORKING_DIRECTORY}")
  execute_process_checked(
    COMMAND git init
    WORKING_DIRECTORY "${SHALLOW_WORKING_DIRECTORY}"
    ERROR_MESSAGE "Failed to initialize ${SHALLOW_NAME} repository"
  )
  execute_process_checked(
    COMMAND git remote add origin "${SHALLOW_REPOSITORY}"
    WORKING_DIRECTORY "${SHALLOW_WORKING_DIRECTORY}"
    ERROR_MESSAGE "Failed to set ${SHALLOW_NAME} remote"
  )
  execute_process_checked(
    COMMAND git fetch --depth=1 origin "${SHALLOW_COMMIT}"
    WORKING_DIRECTORY "${SHALLOW_WORKING_DIRECTORY}"
    ERROR_MESSAGE "Failed to fetch ${SHALLOW_NAME}"
  )
  execute_process_checked(
    COMMAND git reset --hard FETCH_HEAD
    WORKING_DIRECTORY "${SHALLOW_WORKING_DIRECTORY}"
    ERROR_MESSAGE "Failed to checkout ${SHALLOW_NAME}"
  )
endfunction()

# WEBRTC_C_TARGET は必須
if(NOT DEFINED WEBRTC_C_TARGET)
  message(FATAL_ERROR "WEBRTC_C_TARGET is required. Set -DWEBRTC_C_TARGET=...")
endif()

message(STATUS "WEBRTC_C_TARGET: ${WEBRTC_C_TARGET}")
message(STATUS "WEBRTC_BUILD_VERSION: ${WEBRTC_BUILD_VERSION}")

set(WEBRTC_TARGETS_UBUNTU
  "ubuntu-24.04_x86_64"
  "ubuntu-24.04_armv8"
  "ubuntu-22.04_x86_64"
  "ubuntu-22.04_armv8"
)
set(WEBRTC_TARGETS_ARMV8
  "ubuntu-24.04_armv8"
  "ubuntu-22.04_armv8"
)
set(WEBRTC_TARGETS_RASPBERRY_PI
  "raspberry-pi-os_armv8"
)

# WebRTC パッケージの URL を決定
if(WEBRTC_C_TARGET STREQUAL "macos_arm64")
  set(WEBRTC_ARCHIVE_NAME "webrtc.macos_arm64.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "ubuntu-24.04_x86_64")
  set(WEBRTC_ARCHIVE_NAME "webrtc.ubuntu-24.04_x86_64.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "ubuntu-24.04_armv8")
  set(WEBRTC_ARCHIVE_NAME "webrtc.ubuntu-24.04_armv8.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "ubuntu-22.04_x86_64")
  set(WEBRTC_ARCHIVE_NAME "webrtc.ubuntu-22.04_x86_64.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "ubuntu-22.04_armv8")
  set(WEBRTC_ARCHIVE_NAME "webrtc.ubuntu-22.04_armv8.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "raspberry-pi-os_armv8")
  set(WEBRTC_ARCHIVE_NAME "webrtc.raspberry-pi-os_armv8.tar.gz")
elseif(WEBRTC_C_TARGET STREQUAL "windows_x86_64")
  set(WEBRTC_ARCHIVE_NAME "webrtc.windows_x86_64.zip")
else()
  message(FATAL_ERROR "Unsupported target: ${WEBRTC_C_TARGET}")
endif()

set(WEBRTC_URL "${WEBRTC_BASE_URL}/${WEBRTC_BUILD_VERSION}/${WEBRTC_ARCHIVE_NAME}")

# 依存関係のダウンロード先
set(DEPS_DIR "${CMAKE_BINARY_DIR}/_deps")
set(WEBRTC_DIR "${DEPS_DIR}/webrtc")
set(LLVM_DIR "${DEPS_DIR}/llvm")
set(LLVM_CLANG_DIR "${LLVM_DIR}/clang")
set(LLVM_LIBCXX_DIR "${LLVM_DIR}/libcxx")
set(LLVM_TOOLS_DIR "${LLVM_DIR}/tools")
set(LLVM_BUILDTOOLS_DIR "${LLVM_DIR}/buildtools")

# バージョンファイルでキャッシュ管理
set(WEBRTC_CACHE_KEY "${WEBRTC_BUILD_VERSION}:${WEBRTC_C_TARGET}")
set(WEBRTC_VERSION_FILE "${DEPS_DIR}/webrtc.version")
set(LLVM_VERSION_FILE "${DEPS_DIR}/llvm.version")
# WebRTC 向けの Prebuilt Clang バイナリのダウンロードと使用
set(WEBRTC_USE_WEBRTC_PREBUILT_CLANG TRUE)
# WebRTC 向けの libc++ ヘッダーのダウンロードと使用（WebRTC との ABI 互換性のため）
set(WEBRTC_USE_WEBRTC_LIBCXX TRUE)

if(WEBRTC_C_TARGET STREQUAL "windows_x86_64")
  set(WEBRTC_USE_WEBRTC_PREBUILT_CLANG TRUE)
  set(WEBRTC_USE_WEBRTC_LIBCXX FALSE)
endif()

# ARM64 Linux のネイティブビルドでは prebuilt Clang バイナリが x86_64 向けのみ提供のため
# システムの Clang を使用する。
# クロスコンパイル時（WEBRTC_C_SYSROOT が定義されている場合）は x86_64 ホスト上で実行するため
# prebuilt Clang を使用する（デフォルトの TRUE のまま）。
if(WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_ARMV8 AND NOT DEFINED WEBRTC_C_SYSROOT)
  set(WEBRTC_USE_WEBRTC_PREBUILT_CLANG FALSE)
endif()

# WebRTC のダウンロード（project() の前に実行）
set(WEBRTC_NEEDS_DOWNLOAD TRUE)
if(EXISTS "${WEBRTC_VERSION_FILE}")
  file(READ "${WEBRTC_VERSION_FILE}" WEBRTC_INSTALLED_VERSION)
  string(STRIP "${WEBRTC_INSTALLED_VERSION}" WEBRTC_INSTALLED_VERSION)
  if("${WEBRTC_INSTALLED_VERSION}" STREQUAL "${WEBRTC_CACHE_KEY}")
    set(WEBRTC_NEEDS_DOWNLOAD FALSE)
    message(STATUS "WebRTC is already downloaded (key: ${WEBRTC_CACHE_KEY})")
  endif()
endif()

if(WEBRTC_NEEDS_DOWNLOAD)
  message(STATUS "Downloading WebRTC from ${WEBRTC_URL}...")
  file(MAKE_DIRECTORY "${DEPS_DIR}")
  file(REMOVE_RECURSE "${WEBRTC_DIR}")

  set(WEBRTC_ARCHIVE "${DEPS_DIR}/${WEBRTC_ARCHIVE_NAME}")
  file(DOWNLOAD "${WEBRTC_URL}" "${WEBRTC_ARCHIVE}" STATUS DOWNLOAD_STATUS SHOW_PROGRESS)
  list(GET DOWNLOAD_STATUS 0 DOWNLOAD_RESULT)
  if(NOT DOWNLOAD_RESULT EQUAL 0)
    message(FATAL_ERROR "Failed to download WebRTC: ${DOWNLOAD_STATUS}")
  endif()

  message(STATUS "Extracting WebRTC...")
  # アーカイブ内に webrtc/ ディレクトリが含まれているため、DEPS_DIR に展開する
  file(ARCHIVE_EXTRACT INPUT "${WEBRTC_ARCHIVE}" DESTINATION "${DEPS_DIR}")
  file(REMOVE "${WEBRTC_ARCHIVE}")
  if(NOT EXISTS "${WEBRTC_DIR}/VERSIONS")
    message(FATAL_ERROR "WebRTC archive does not contain expected file: ${WEBRTC_DIR}/VERSIONS")
  endif()
  file(WRITE "${WEBRTC_VERSION_FILE}" "${WEBRTC_CACHE_KEY}")
  message(STATUS "WebRTC download complete")
endif()

# WebRTC のパスを設定
set(WEBRTC_INCLUDE_DIR "${WEBRTC_DIR}/include")
set(WEBRTC_LIBRARY_DIR "${WEBRTC_DIR}/lib")

# WebRTC パッケージの VERSIONS ファイルから LLVM 関連の情報を読み込む
file(STRINGS "${WEBRTC_DIR}/VERSIONS" VERSIONS_LINES)
foreach(line IN LISTS VERSIONS_LINES)
  if(line MATCHES "^([A-Z_]+)=(.*)$")
    set(WEBRTC_${CMAKE_MATCH_1} "${CMAKE_MATCH_2}")
  endif()
endforeach()

message(STATUS "LLVM tools commit: ${WEBRTC_WEBRTC_SRC_TOOLS_COMMIT}")
message(STATUS "LLVM libcxx commit: ${WEBRTC_WEBRTC_SRC_THIRD_PARTY_LIBCXX_SRC_COMMIT}")

# LLVM バージョン文字列
set(LLVM_VERSION_STRING "${WEBRTC_WEBRTC_SRC_TOOLS_COMMIT}.${WEBRTC_WEBRTC_SRC_BUILDTOOLS_COMMIT}.${WEBRTC_WEBRTC_SRC_THIRD_PARTY_LIBCXX_SRC_COMMIT}")
set(LLVM_CACHE_KEY "${LLVM_VERSION_STRING}:${WEBRTC_C_TARGET}")

# LLVM のダウンロード（project() の前に実行）
set(LLVM_NEEDS_INSTALL TRUE)
if((WEBRTC_USE_WEBRTC_PREBUILT_CLANG OR WEBRTC_USE_WEBRTC_LIBCXX) AND EXISTS "${LLVM_VERSION_FILE}")
  file(READ "${LLVM_VERSION_FILE}" LLVM_INSTALLED_VERSION)
  string(STRIP "${LLVM_INSTALLED_VERSION}" LLVM_INSTALLED_VERSION)
  if("${LLVM_INSTALLED_VERSION}" STREQUAL "${LLVM_CACHE_KEY}")
    set(LLVM_NEEDS_INSTALL FALSE)
    message(STATUS "LLVM is already installed")
  endif()
endif()

if((WEBRTC_USE_WEBRTC_PREBUILT_CLANG OR WEBRTC_USE_WEBRTC_LIBCXX) AND LLVM_NEEDS_INSTALL)
  message(STATUS "Installing LLVM...")
  file(REMOVE_RECURSE "${LLVM_DIR}")
  file(MAKE_DIRECTORY "${LLVM_DIR}")

  if(WEBRTC_USE_WEBRTC_PREBUILT_CLANG)
    # tools を shallow clone
    message(STATUS "Cloning tools...")
    git_shallow_clone(
      NAME "tools"
      WORKING_DIRECTORY "${LLVM_TOOLS_DIR}"
      REPOSITORY "${WEBRTC_WEBRTC_SRC_TOOLS_URL}"
      COMMIT "${WEBRTC_WEBRTC_SRC_TOOLS_COMMIT}"
    )

    # update.py を実行して Clang をダウンロード
    message(STATUS "Downloading Clang via update.py...")
    execute_process_checked(
      COMMAND python3 "${LLVM_TOOLS_DIR}/clang/scripts/update.py" --output-dir "${LLVM_CLANG_DIR}"
      ERROR_MESSAGE "Failed to download Clang"
    )
  endif()

  if(WEBRTC_USE_WEBRTC_LIBCXX)
    # libcxx を shallow clone
    message(STATUS "Cloning libcxx...")
    git_shallow_clone(
      NAME "libcxx"
      WORKING_DIRECTORY "${LLVM_LIBCXX_DIR}"
      REPOSITORY "${WEBRTC_WEBRTC_SRC_THIRD_PARTY_LIBCXX_SRC_URL}"
      COMMIT "${WEBRTC_WEBRTC_SRC_THIRD_PARTY_LIBCXX_SRC_COMMIT}"
    )

    # buildtools を shallow clone
    message(STATUS "Cloning buildtools...")
    git_shallow_clone(
      NAME "buildtools"
      WORKING_DIRECTORY "${LLVM_BUILDTOOLS_DIR}"
      REPOSITORY "${WEBRTC_WEBRTC_SRC_BUILDTOOLS_URL}"
      COMMIT "${WEBRTC_WEBRTC_SRC_BUILDTOOLS_COMMIT}"
    )

    # __config_site と __assertion_handler をコピー
    message(STATUS "Copying libc++ config files...")
    file(COPY "${LLVM_BUILDTOOLS_DIR}/third_party/libc++/__config_site" DESTINATION "${LLVM_LIBCXX_DIR}/include")
    file(COPY "${LLVM_BUILDTOOLS_DIR}/third_party/libc++/__assertion_handler" DESTINATION "${LLVM_LIBCXX_DIR}/include")
    if(NOT EXISTS "${LLVM_LIBCXX_DIR}/include/__config_site")
      message(FATAL_ERROR "Failed to copy __config_site")
    endif()
    if(NOT EXISTS "${LLVM_LIBCXX_DIR}/include/__assertion_handler")
      message(FATAL_ERROR "Failed to copy __assertion_handler")
    endif()
  endif()

  file(WRITE "${LLVM_VERSION_FILE}" "${LLVM_CACHE_KEY}")
  message(STATUS "LLVM installation complete")
endif()

if(WEBRTC_USE_WEBRTC_PREBUILT_CLANG)
  # Prebuilt Clang コンパイラのパスを設定（project() の前）
  set(CMAKE_C_COMPILER "${LLVM_CLANG_DIR}/bin/clang" CACHE FILEPATH "" FORCE)
  set(CMAKE_CXX_COMPILER "${LLVM_CLANG_DIR}/bin/clang++" CACHE FILEPATH "" FORCE)
  message(STATUS "CMAKE_C_COMPILER: ${CMAKE_C_COMPILER}")
  message(STATUS "CMAKE_CXX_COMPILER: ${CMAKE_CXX_COMPILER}")
elseif(WEBRTC_USE_WEBRTC_LIBCXX)
  # libc++ を使用するがプリビルト Clang は使わない場合（ARM64 Linux）
  # ABI 互換性のためシステムの Clang を使用する
  find_program(SYSTEM_CLANG clang)
  find_program(SYSTEM_CLANGPP clang++)
  if(SYSTEM_CLANG AND SYSTEM_CLANGPP)
    set(CMAKE_C_COMPILER "${SYSTEM_CLANG}" CACHE FILEPATH "" FORCE)
    set(CMAKE_CXX_COMPILER "${SYSTEM_CLANGPP}" CACHE FILEPATH "" FORCE)
    message(STATUS "Using system Clang: ${SYSTEM_CLANG}")
    message(STATUS "Using system Clang++: ${SYSTEM_CLANGPP}")
  else()
    message(FATAL_ERROR "System Clang is required for ARM64 Linux but not found. Install with: apt install clang")
  endif()
else()
  message(STATUS "WEBRTC_USE_WEBRTC_PREBUILT_CLANG is OFF for target: ${WEBRTC_C_TARGET}")
endif()

if(WEBRTC_USE_WEBRTC_LIBCXX)
  set(LIBCXX_INCLUDE_DIR "${LLVM_LIBCXX_DIR}/include")
  message(STATUS "LIBCXX_INCLUDE_DIR: ${LIBCXX_INCLUDE_DIR}")
endif()

# ARM64 クロスコンパイル設定（project() の前に配置が必要）
# WEBRTC_C_SYSROOT が定義されている場合、x86_64 ホストから ARM64 ターゲットへのクロスコンパイルを行う
if((WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_RASPBERRY_PI OR WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_ARMV8) AND DEFINED WEBRTC_C_SYSROOT)
  set(CMAKE_SYSTEM_NAME Linux)
  set(CMAKE_SYSTEM_PROCESSOR aarch64)
  set(CMAKE_SYSROOT "${WEBRTC_C_SYSROOT}")
  set(CMAKE_C_COMPILER_TARGET aarch64-linux-gnu)
  set(CMAKE_CXX_COMPILER_TARGET aarch64-linux-gnu)
  set(CMAKE_FIND_ROOT_PATH "${WEBRTC_C_SYSROOT}")
  set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
  set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY BOTH)
  set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE BOTH)
  set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE BOTH)
  set(CMAKE_AR "${LLVM_CLANG_DIR}/bin/llvm-ar" CACHE FILEPATH "" FORCE)
  # クロスコンパイル時はシステムの ld ではなく LLD を使用する
  set(CMAKE_EXE_LINKER_FLAGS "-fuse-ld=lld" CACHE STRING "" FORCE)
  set(CMAKE_SHARED_LINKER_FLAGS "-fuse-ld=lld" CACHE STRING "" FORCE)
endif()

# ここで project() を呼び出す
project(webrtc_c C CXX)

set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE NEVER)

find_package(Threads REQUIRED)

set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

add_library(webrtc_c
  STATIC
    src/webrtc_c/api/audio/audio_device.cc
    src/webrtc_c/api/audio/audio_device_defines.cc
    src/webrtc_c/api/audio/audio_processing.cc
    src/webrtc_c/api/audio_codecs/audio_decoder_factory.cc
    src/webrtc_c/api/audio_codecs/audio_encoder_factory.cc
    src/webrtc_c/api/data_channel_interface.cc
    src/webrtc_c/api/jsep.cc
    src/webrtc_c/api/environment.cc
    src/webrtc_c/api/media_types.cc
    src/webrtc_c/api/media_stream_interface.cc
    src/webrtc_c/api/peer_connection_interface.cc
    src/webrtc_c/api/ref_count.cc
    src/webrtc_c/api/rtc_error.cc
    src/webrtc_c/api/set_local_description_observer_interface.cc
    src/webrtc_c/api/set_remote_description_observer_interface.cc
    src/webrtc_c/api/rtc_event_log.cc
    src/webrtc_c/api/rtp_parameters.cc
    src/webrtc_c/api/rtp_receiver_interface.cc
    src/webrtc_c/api/rtp_sender_interface.cc
    src/webrtc_c/api/rtp_transceiver_direction.cc
    src/webrtc_c/api/rtp_transceiver_interface.cc
    src/webrtc_c/api/stats/rtc_stats_report.cc
    src/webrtc_c/api/video/i420_buffer.cc
    src/webrtc_c/api/video/video_frame.cc
    src/webrtc_c/api/video/video_sink_interface.cc
    src/webrtc_c/api/video/video_source_interface.cc
    src/webrtc_c/api/video_codecs/sdp_video_format.cc
    src/webrtc_c/api/video_codecs/video_decoder_factory.cc
    src/webrtc_c/api/video_codecs/video_encoder_factory.cc
    src/webrtc_c/libyuv.cc
    src/webrtc_c/media/base/adapted_video_track_source.cc
    src/webrtc_c/rtc_base/crypto_random.cc
    src/webrtc_c/rtc_base/logging.cc
    src/webrtc_c/rtc_base/ssl_adapter.cc
    src/webrtc_c/rtc_base/thread.cc
    src/webrtc_c/rtc_base/time_utils.cc
    src/webrtc_c/rtc_base/timestamp_aligner.cc
    src/webrtc_c/std.cc
)
add_executable(whip_cpp src/whip.cpp)
add_executable(whep_cpp src/whep.cpp)
set(WEBRTC_CPP_TARGETS webrtc_c whip_cpp whep_cpp)
set(WEBRTC_C_TARGETS whip_c whep_c)

foreach(target IN LISTS WEBRTC_CPP_TARGETS)
  set_target_properties(${target} PROPERTIES CXX_STANDARD 20 C_STANDARD 20)
endforeach()

foreach(target IN LISTS WEBRTC_CPP_TARGETS)
  target_include_directories(${target}
    PRIVATE
      ${WEBRTC_INCLUDE_DIR}
      ${WEBRTC_INCLUDE_DIR}/third_party/abseil-cpp
      ${WEBRTC_INCLUDE_DIR}/third_party/boringssl/src/include
      ${WEBRTC_INCLUDE_DIR}/third_party/libyuv/include
      ${WEBRTC_INCLUDE_DIR}/third_party/zlib
  )
endforeach()

foreach(target IN LISTS WEBRTC_CPP_TARGETS)
  target_link_libraries(${target}
    PRIVATE
      ${WEBRTC_LIBRARY_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}webrtc${CMAKE_STATIC_LIBRARY_SUFFIX}
      Threads::Threads
  )
endforeach()

set(BUNDLE_STATIC_LIBS ${WEBRTC_LIBRARY_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}webrtc${CMAKE_STATIC_LIBRARY_SUFFIX})

if(WEBRTC_USE_WEBRTC_LIBCXX)
  # libc++ の設定（WebRTC と同じ libc++ ヘッダーを使用）
  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_compile_options(${target}
      PRIVATE
        "$<$<COMPILE_LANGUAGE:CXX>:-nostdinc++>"
        "$<$<COMPILE_LANGUAGE:CXX>:-isystem${LIBCXX_INCLUDE_DIR}>"
        "$<$<COMPILE_LANGUAGE:OBJCXX>:-nostdinc++>"
        "$<$<COMPILE_LANGUAGE:OBJCXX>:-isystem${LIBCXX_INCLUDE_DIR}>"
    )
  endforeach()
endif()

# 指定したライブラリを自身の静的ライブラリにバンドルする
function(bundle_static_library target static_libs bundled_target)
  file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/bundled)
  set(bundled_tgt_full_name
    ${CMAKE_BINARY_DIR}/bundled/${CMAKE_STATIC_LIBRARY_PREFIX}${target}${CMAKE_STATIC_LIBRARY_SUFFIX})

  if (MSVC)
    add_custom_command(
      COMMAND ${CMAKE_AR} /NOLOGO /OUT:${bundled_tgt_full_name} $<TARGET_FILE:${target}> ${static_libs}
      OUTPUT ${bundled_tgt_full_name}
      DEPENDS $<TARGET_FILE:${target}> ${static_libs}
      VERBATIM
      COMMENT "Bundling libs: ${static_libs} to $<TARGET_FILE:${target}>")
  elseif (APPLE)
    add_custom_command(
      COMMAND libtool -static -o ${bundled_tgt_full_name} "$<TARGET_FILE:${target}>" ${static_libs}
      OUTPUT ${bundled_tgt_full_name}
      DEPENDS $<TARGET_FILE:${target}> ${static_libs}
      VERBATIM
      COMMENT "Bundling libs: ${static_libs} to $<TARGET_FILE:${target}>")
  else ()
    file(WRITE  ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in "CREATE ${bundled_tgt_full_name}\n" )
    file(APPEND ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in "ADDLIB $<TARGET_FILE:${target}>\n")
    foreach(lib IN LISTS static_libs)
      file(APPEND ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in "ADDLIB ${lib}\n")
    endforeach()
    file(APPEND ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in "SAVE\n")
    file(APPEND ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in "END\n")
    file(GENERATE
      OUTPUT ${CMAKE_BINARY_DIR}/${bundled_target}.ar
      INPUT ${CMAKE_BINARY_DIR}/${bundled_target}.ar.in)

    add_custom_command(
      COMMAND ${CMAKE_AR} -M < ${CMAKE_BINARY_DIR}/${bundled_target}.ar
      OUTPUT ${bundled_tgt_full_name}
      DEPENDS $<TARGET_FILE:${target}> ${static_libs}
      VERBATIM
      COMMENT "Bundling libs: ${static_libs} to $<TARGET_FILE:${target}>")
  endif()

  add_custom_target(${bundled_target}_bundling ALL DEPENDS ${bundled_tgt_full_name})
  add_dependencies(${bundled_target}_bundling ${target})

  add_library(${bundled_target} STATIC IMPORTED)
  set_target_properties(${bundled_target}
    PROPERTIES
      IMPORTED_LOCATION ${bundled_tgt_full_name}
      INTERFACE_INCLUDE_DIRECTORIES $<TARGET_PROPERTY:${target},INTERFACE_INCLUDE_DIRECTORIES>)
  add_dependencies(${bundled_target} ${bundled_target}_bundling)
endfunction()

if (WEBRTC_C_TARGET STREQUAL "windows_x86_64")
  # 文字コードを utf-8 として扱うのと、シンボルテーブル数を増やす
  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_compile_options(${target} PRIVATE /utf-8 /bigobj)
    set_target_properties(${target}
      PROPERTIES
        # CRTライブラリを静的リンクさせる
        MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>"
    )
    target_link_libraries(${target}
      PRIVATE
      #  dbghelp.lib
      #  delayimp.lib
      #  dnsapi.lib
      #  msimg32.lib
      #  oleaut32.lib
      #  psapi.lib
      #  shell32.lib
      #  shlwapi.lib
      #  usp10.lib
      #  version.lib
      #  wininet.lib
        winmm.lib
        ws2_32.lib
      #  amstrmid.lib
        Strmiids.lib
      #  crypt32.lib
        dmoguids.lib
        iphlpapi.lib
        msdmo.lib
        Secur32.lib
        wmcodecdspuuid.lib
    )
    target_compile_definitions(${target}
      PRIVATE
        _CONSOLE
        _WIN32_WINNT=0x0A00
        WEBRTC_WIN
        NOMINMAX
        WIN32_LEAN_AND_MEAN
        UNICODE
        _UNICODE
        _USE_MATH_DEFINES
        RTC_ENABLE_H265
        _ITERATOR_DEBUG_LEVEL=0
    )
  endforeach()
elseif (WEBRTC_C_TARGET STREQUAL "macos_arm64")
  enable_language(OBJCXX)

  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_compile_options(${target} PRIVATE -fconstant-string-class=NSConstantString)
    target_link_options(${target} PRIVATE -ObjC)
    target_compile_definitions(${target}
      PRIVATE
        WEBRTC_POSIX
        WEBRTC_MAC
        _LIBCPP_ABI_NAMESPACE=Cr
        _LIBCPP_ABI_VERSION=2
        _LIBCPP_DISABLE_AVAILABILITY
        _LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS
        _LIBCXXABI_DISABLE_VISIBILITY_ANNOTATIONS
        _LIBCPP_ENABLE_NODISCARD
        _LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE
    )
    set_target_properties(${target} PROPERTIES CXX_VISIBILITY_PRESET hidden)
  endforeach()

  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_link_libraries(${target}
      PRIVATE
        #"-framework Foundation"
        "-framework AVFoundation"
        #"-framework CoreServices"
        #"-framework CoreFoundation"
        #"-framework AudioUnit"
        "-framework AudioToolbox"
        "-framework CoreAudio"
        "-framework QuartzCore"
        #"-framework CoreGraphics"
        "-framework CoreMedia"
        #"-framework CoreVideo"
        "-framework VideoToolbox"
        "-framework AppKit"
        "-framework Metal"
        "-framework MetalKit"
        "-framework OpenGL"
        "-framework IOSurface"
        "-framework ScreenCaptureKit"
    )
  endforeach()

elseif (WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_UBUNTU)
  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_compile_definitions(${target}
      PRIVATE
        WEBRTC_POSIX
        WEBRTC_LINUX
        _LIBCPP_ABI_NAMESPACE=Cr
        _LIBCPP_ABI_VERSION=2
        _LIBCPP_DISABLE_AVAILABILITY
        _LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE
        # https://github.com/boostorg/container_hash/issues/22 と同じ問題が clang-15 でも起きるので、これを手動で定義して回避する
        BOOST_NO_CXX98_FUNCTION_BASE
        RTC_ENABLE_H265
    )
    target_compile_options(${target} PRIVATE -Wno-nullability-completeness)
    set_target_properties(${target} PROPERTIES POSITION_INDEPENDENT_CODE ON)
    target_link_libraries(${target}
      PRIVATE
        X11
        #Xau
        #Xdmcp
        #Xtst
        #xcb
        #plds4
        #Xext
        #expat
        dl
        #nss3
        #nssutil3
        #plc4
        #nspr4
        rt
        Threads::Threads
    )
  endforeach()

elseif (WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_RASPBERRY_PI)
  foreach(target IN LISTS WEBRTC_CPP_TARGETS)
    target_compile_definitions(${target}
      PRIVATE
        WEBRTC_POSIX
        WEBRTC_LINUX
        _LIBCPP_ABI_NAMESPACE=Cr
        _LIBCPP_ABI_VERSION=2
        _LIBCPP_DISABLE_AVAILABILITY
        _LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE
        BOOST_NO_CXX98_FUNCTION_BASE
        RTC_ENABLE_H265
    )
    target_compile_options(${target} PRIVATE -Wno-nullability-completeness)
    set_target_properties(${target} PROPERTIES POSITION_INDEPENDENT_CODE ON)
    target_link_libraries(${target}
      PRIVATE
        X11
        dl
        rt
        Threads::Threads
    )
  endforeach()

endif()

# 静的ライブラリを webrtc_c.lib に含める
bundle_static_library(webrtc_c "${BUNDLE_STATIC_LIBS}" bundled_webrtc_c)

# bundled_webrtc_c をインストールする
install(FILES ${CMAKE_BINARY_DIR}/bundled/${CMAKE_STATIC_LIBRARY_PREFIX}webrtc_c${CMAKE_STATIC_LIBRARY_SUFFIX} DESTINATION lib)

# bundled_webrtc_c を使った whip_c ターゲットを作成する
add_executable(whip_c src/whip.c)
add_executable(whep_c src/whep.c)
foreach(target IN LISTS WEBRTC_C_TARGETS)
  target_include_directories(${target}
    PRIVATE
      ${WEBRTC_INCLUDE_DIR}
      ${WEBRTC_INCLUDE_DIR}/third_party/abseil-cpp
      ${WEBRTC_INCLUDE_DIR}/third_party/boringssl/src/include
      ${WEBRTC_INCLUDE_DIR}/third_party/libyuv/include
      ${WEBRTC_INCLUDE_DIR}/third_party/zlib
  )
  target_link_libraries(${target} PRIVATE bundled_webrtc_c)
  if(NOT WEBRTC_C_TARGET STREQUAL "windows_x86_64")
    target_link_libraries(${target} PRIVATE m)
  endif()
endforeach()

# プラットフォーム固有のリンク設定 (WEBRTC_C_TARGETS)
if (WEBRTC_C_TARGET STREQUAL "windows_x86_64")
  foreach(target IN LISTS WEBRTC_C_TARGETS)
    target_link_libraries(${target}
      PRIVATE
        winmm.lib
        ws2_32.lib
        Strmiids.lib
        dmoguids.lib
        iphlpapi.lib
        msdmo.lib
        Secur32.lib
        wmcodecdspuuid.lib
    )
    target_compile_definitions(${target}
      PRIVATE
        _CONSOLE
        _WIN32_WINNT=0x0A00
        WEBRTC_WIN
        NOMINMAX
        WIN32_LEAN_AND_MEAN
        UNICODE
        _UNICODE
        _USE_MATH_DEFINES
        RTC_ENABLE_H265
    )
  endforeach()
elseif (WEBRTC_C_TARGET STREQUAL "macos_arm64")
  foreach(target IN LISTS WEBRTC_C_TARGETS)
    target_link_options(${target} PRIVATE -ObjC)
    target_link_libraries(${target}
      PRIVATE
        "-framework AVFoundation"
        "-framework AudioToolbox"
        "-framework CoreAudio"
        "-framework QuartzCore"
        "-framework CoreMedia"
        "-framework VideoToolbox"
        "-framework AppKit"
        "-framework Metal"
        "-framework MetalKit"
        "-framework OpenGL"
        "-framework IOSurface"
        "-framework ScreenCaptureKit"
    )
  endforeach()
elseif (WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_UBUNTU)
  foreach(target IN LISTS WEBRTC_C_TARGETS)
    target_link_libraries(${target} PRIVATE X11 dl rt Threads::Threads)
  endforeach()
elseif (WEBRTC_C_TARGET IN_LIST WEBRTC_TARGETS_RASPBERRY_PI)
  foreach(target IN LISTS WEBRTC_C_TARGETS)
    target_link_libraries(${target} PRIVATE X11 dl rt Threads::Threads)
  endforeach()
endif()
