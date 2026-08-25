#include "dependency_descriptor.h"

#include <api/transport/rtp/dependency_descriptor.h>

#include "../../../common.h"

extern "C" {
WEBRTC_EXPORT const int webrtc_DecodeTargetIndication_NotPresent =
    static_cast<int>(webrtc::DecodeTargetIndication::kNotPresent);
WEBRTC_EXPORT const int webrtc_DecodeTargetIndication_Discardable =
    static_cast<int>(webrtc::DecodeTargetIndication::kDiscardable);
WEBRTC_EXPORT const int webrtc_DecodeTargetIndication_Switch =
    static_cast<int>(webrtc::DecodeTargetIndication::kSwitch);
WEBRTC_EXPORT const int webrtc_DecodeTargetIndication_Required =
    static_cast<int>(webrtc::DecodeTargetIndication::kRequired);
}
