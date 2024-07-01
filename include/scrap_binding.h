#ifndef SCRAP_BINDING_H
#define SCRAP_BINDING_H

/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "scrap_binding_tools.h"

typedef enum ScrapCaptureResult {
  ScrapCaptureSuccessful = 0,
  ScrapCaptureShouldSkip = 1,
  ScrapCaptureUnknown = 2,
  ScrapCaptureShouldReset = 3,
} ScrapCaptureResult;

typedef struct ScrapSize {
  int width;
  int height;
} ScrapSize;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

SCRAP_API int32_t scrap_get_display_num(void);

SCRAP_API void scrap_get_display_size(size_t index, struct ScrapSize *size);

SCRAP_API void *scrap_create_capturer(int32_t index);

SCRAP_API void scrap_free_capturer(void *capturer);

SCRAP_API enum ScrapCaptureResult scrap_get_frame(void *capturer, void *dst, size_t size);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* SCRAP_BINDING_H */
