#ifndef SCRAP_BINDING_TOOLS_H
#define SCRAP_BINDING_TOOLS_H

#if defined _WIN32 || defined __CYGWIN__
#define SCRAP_HELPER_DLL_IMPORT __declspec(dllimport)
#else
#if __GNUC__ >= 4
#define SCRAP_HELPER_DLL_IMPORT __attribute__ ((visibility ("default")))
#else
#define SCRAP_HELPER_DLL_IMPORT
#endif
#endif

#define SCRAP_API SCRAP_HELPER_DLL_IMPORT

#endif // SCRAP_BINDING_TOOLS_H
