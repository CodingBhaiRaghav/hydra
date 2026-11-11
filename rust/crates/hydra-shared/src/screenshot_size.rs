//! Port of `src/main/services/screenshot-size.ts`.

pub const MAX_SCREENSHOT_HEIGHT: i64 = 1080;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScreenshotSize {
    pub width: i64,
    pub height: i64,
}

pub fn fit_screenshot_to_1080p(size: ScreenshotSize) -> ScreenshotSize {
    if size.height <= MAX_SCREENSHOT_HEIGHT {
        return size;
    }

    let scale = MAX_SCREENSHOT_HEIGHT as f64 / size.height as f64;

    ScreenshotSize {
        width: (size.width as f64 * scale).round() as i64,
        height: MAX_SCREENSHOT_HEIGHT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_4k_screenshots_down_to_1080p() {
        assert_eq!(
            fit_screenshot_to_1080p(ScreenshotSize {
                width: 3840,
                height: 2160
            }),
            ScreenshotSize {
                width: 1920,
                height: 1080
            }
        );
    }

    #[test]
    fn preserves_the_aspect_ratio_of_ultrawide_screenshots() {
        assert_eq!(
            fit_screenshot_to_1080p(ScreenshotSize {
                width: 3440,
                height: 1440
            }),
            ScreenshotSize {
                width: 2580,
                height: 1080
            }
        );
    }

    #[test]
    fn leaves_1080p_screenshots_unchanged() {
        let size = ScreenshotSize {
            width: 1920,
            height: 1080,
        };
        assert_eq!(fit_screenshot_to_1080p(size), size);
    }

    #[test]
    fn does_not_upscale_screenshots_below_1080p() {
        let size = ScreenshotSize {
            width: 1280,
            height: 720,
        };
        assert_eq!(fit_screenshot_to_1080p(size), size);
    }
}
