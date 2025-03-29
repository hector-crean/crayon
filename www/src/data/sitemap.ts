import { AppRouterEntry } from "@/bindings/AppRouterEntry";
export const appRouterEntries: AppRouterEntry[] = [
  {
    type: "File",
    file_kind: "Other",
    path_segment: "favicon.ico",
    relative_path: "favicon.ico",
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "auth",
    relative_path: "auth",
    children: [
      {
        type: "Directory",
        directory_kind: "StaticRoute",
        path_segment: "login",
        relative_path: "auth/login",
        children: [
          {
            type: "File",
            file_kind: "Page",
            path_segment: "page.tsx",
            relative_path: "auth/login/page.tsx",
          },
        ],
      },
    ],
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "input",
    relative_path: "input",
    children: [
      {
        type: "File",
        file_kind: "Page",
        path_segment: "page.tsx",
        relative_path: "input/page.tsx",
      },
    ],
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "output",
    relative_path: "output",
    children: [
      {
        type: "File",
        file_kind: "Page",
        path_segment: "page.tsx",
        relative_path: "output/page.tsx",
      },
    ],
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "details",
    relative_path: "details",
    children: [
      {
        type: "File",
        file_kind: "Layout",
        path_segment: "layout.tsx",
        relative_path: "details/layout.tsx",
      },
      {
        type: "File",
        file_kind: "Page",
        path_segment: "page.tsx",
        relative_path: "details/page.tsx",
      },
    ],
  },
  {
    type: "File",
    file_kind: "Layout",
    path_segment: "layout.tsx",
    relative_path: "layout.tsx",
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "api",
    relative_path: "api",
    children: [
      {
        type: "Directory",
        directory_kind: "DynamicRouteWithParams",
        path_segment: "[auth]",
        relative_path: "api/[auth]",
        children: [
          {
            type: "Directory",
            directory_kind: "StaticRoute",
            path_segment: "[...nextauth]",
            relative_path: "api/[auth]/[...nextauth]",
            children: [
              {
                type: "File",
                file_kind: "Route",
                path_segment: "route.ts",
                relative_path: "api/[auth]/[...nextauth]/route.ts",
              },
            ],
          },
        ],
      },
    ],
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "fonts",
    relative_path: "fonts",
    children: [
      {
        type: "Directory",
        directory_kind: "StaticRoute",
        path_segment: "iosevka",
        relative_path: "fonts/iosevka",
        children: [
          {
            type: "File",
            file_kind: "Other",
            path_segment: "IosevkaSS05.css",
            relative_path: "fonts/iosevka/IosevkaSS05.css",
          },
          {
            type: "Directory",
            directory_kind: "StaticRoute",
            path_segment: "TTF",
            relative_path: "fonts/iosevka/TTF",
            children: [
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLightItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtraLightItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBold.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraBold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-BoldOblique.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-BoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBold.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ExtendedBold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBoldItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedSemiBoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Light.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Light.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Medium.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Medium.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThin.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ExtendedThin.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMedium.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedMedium.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMediumOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedMediumOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLight.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedLight.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMediumItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedMediumItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBoldItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedBoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Heavy.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Heavy.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThinItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedThinItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLight.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraLight.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBoldOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-SemiBoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Regular.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Regular.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBold.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedSemiBold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavy.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedHeavy.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBoldItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-SemiBoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-MediumOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-MediumOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ThinOblique.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ThinOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-MediumItalic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-MediumItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLightItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraLightItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBoldItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtraBoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavyItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedHeavyItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLightItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedLightItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBold.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-SemiBold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBoldOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedBoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Bold.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Bold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLightOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedLightOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBoldOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedSemiBoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBoldOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraBoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBold.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ExtraBold.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBoldItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraBoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Thin.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Thin.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-LightOblique.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-LightOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLightOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedExtraLightOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Oblique.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Oblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-HeavyOblique.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-HeavyOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLightOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtraLightOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Extended.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Extended.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ThinItalic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ThinItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedItalic.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-LightItalic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-LightItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavyOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedHeavyOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-HeavyItalic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-HeavyItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLight.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-ExtraLight.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBoldOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtraBoldOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThinOblique.ttf",
                relative_path:
                  "fonts/iosevka/TTF/IosevkaSS05-ExtendedThinOblique.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-BoldItalic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-BoldItalic.ttf",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Italic.ttf",
                relative_path: "fonts/iosevka/TTF/IosevkaSS05-Italic.ttf",
              },
            ],
          },
          {
            type: "Directory",
            directory_kind: "StaticRoute",
            path_segment: "WOFF2",
            relative_path: "fonts/iosevka/WOFF2",
            children: [
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMediumItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedMediumItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Regular.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Regular.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBold.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraBold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThinOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedThinOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavy.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedHeavy.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBold.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-SemiBold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLightItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraLightItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLightItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedLightItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraBoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedBoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBold.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedBold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-MediumOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-MediumOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ThinOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ThinOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-MediumItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-MediumItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Bold.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Bold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-BoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-BoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-LightItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-LightItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThin.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedThin.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Light.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Light.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Medium.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Medium.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Thin.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Thin.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavyItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedHeavyItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLight.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraLight.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBold.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraBold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMediumOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedMediumOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Extended.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Extended.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBold.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedSemiBold.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-HeavyOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-HeavyOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedSemiBoldItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLight.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedLight.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedSemiBoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedSemiBoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLight.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraLight.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraBoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraBoldItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedMedium.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedMedium.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedLightOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedLightOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Oblique.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Oblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraBoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-SemiBoldItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLightItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraLightItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedThinItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedThinItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-SemiBoldOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-SemiBoldOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-HeavyItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-HeavyItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-BoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-BoldItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraBoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraBoldItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ThinItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ThinItalic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedHeavyOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedHeavyOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedExtraLightOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedExtraLightOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Italic.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Italic.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtraLightOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtraLightOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-Heavy.woff2",
                relative_path: "fonts/iosevka/WOFF2/IosevkaSS05-Heavy.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-LightOblique.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-LightOblique.woff2",
              },
              {
                type: "File",
                file_kind: "Other",
                path_segment: "IosevkaSS05-ExtendedBoldItalic.woff2",
                relative_path:
                  "fonts/iosevka/WOFF2/IosevkaSS05-ExtendedBoldItalic.woff2",
              },
            ],
          },
        ],
      },
    ],
  },
  {
    type: "File",
    file_kind: "Page",
    path_segment: "page.tsx",
    relative_path: "page.tsx",
  },
  {
    type: "Directory",
    directory_kind: "StaticRoute",
    path_segment: "design-system",
    relative_path: "design-system",
    children: [
      {
        type: "File",
        file_kind: "Page",
        path_segment: "page.tsx",
        relative_path: "design-system/page.tsx",
      },
    ],
  },
  {
    type: "File",
    file_kind: "Other",
    path_segment: "globals.css",
    relative_path: "globals.css",
  },
  {
    type: "Directory",
    directory_kind: "RouteGroup",
    path_segment: "(admin)",
    relative_path: "(admin)",
    children: [
      {
        type: "Directory",
        directory_kind: "StaticRoute",
        path_segment: "dashboard",
        relative_path: "(admin)/dashboard",
        children: [
          {
            type: "File",
            file_kind: "Page",
            path_segment: "page.tsx",
            relative_path: "(admin)/dashboard/page.tsx",
          },
        ],
      },
      {
        type: "File",
        file_kind: "Layout",
        path_segment: "layout.tsx",
        relative_path: "(admin)/layout.tsx",
      },
    ],
  },
];
