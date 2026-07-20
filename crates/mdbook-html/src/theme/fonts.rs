pub(crate) static CSS: &[u8] = include_bytes!("../../front-end/fonts/fonts.css");
// An array of (file_name, file_contents) pairs
pub(crate) static LICENSES: [(&str, &[u8]); 2] = [
    (
        "fonts/OPEN-SANS-LICENSE.txt",
        include_bytes!("../../front-end/fonts/OPEN-SANS-LICENSE.txt"),
    ),
    (
        "fonts/SOURCE-CODE-PRO-LICENSE.txt",
        include_bytes!("../../front-end/fonts/SOURCE-CODE-PRO-LICENSE.txt"),
    ),
];
pub(crate) static ROBOTO_LICENSE: (&str, &[u8]) = (
    "fonts/ROBOTO-LICENSE.txt",
    include_bytes!("../../front-end/fonts/ROBOTO-LICENSE.txt"),
);
pub(crate) static OPENDYSLEXIC_LICENSE: (&str, &[u8]) = (
    "fonts/OPENDYSLEXIC-LICENSE.txt",
    include_bytes!("../../front-end/fonts/OPENDYSLEXIC-LICENSE.txt"),
);
// An array of (file_name, file_contents) pairs
pub(crate) static OPEN_SANS: [(&str, &[u8]); 10] = [
    (
        "fonts/open-sans-v17-all-charsets-300.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-300.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-300italic.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-300italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-regular.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-regular.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-italic.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-600.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-600.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-600italic.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-600italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-700.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-700.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-700italic.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-700italic.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-800.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-800.woff2"),
    ),
    (
        "fonts/open-sans-v17-all-charsets-800italic.woff2",
        include_bytes!("../../front-end/fonts/open-sans-v17-all-charsets-800italic.woff2"),
    ),
];

pub(crate) static ROBOTO: [(&str, &[u8]); 2] = [
    (
        "fonts/Roboto-VariableFont_wdth,wght.ttf",
        include_bytes!("../../front-end/fonts/Roboto-VariableFont_wdth,wght.ttf"),
    ),
    (
        "fonts/Roboto-Italic-VariableFont_wdth,wght.ttf",
        include_bytes!("../../front-end/fonts/Roboto-Italic-VariableFont_wdth,wght.ttf"),
    ),
];

pub(crate) static OPENDYSLEXIC: [(&str, &[u8]); 5] = [
    (
        "fonts/OpenDyslexic-Regular.woff2",
        include_bytes!("../../front-end/fonts/OpenDyslexic-Regular.woff2"),
    ),
    (
        "fonts/OpenDyslexic-Italic.woff2",
        include_bytes!("../../front-end/fonts/OpenDyslexic-Italic.woff2"),
    ),
    (
        "fonts/OpenDyslexic-Bold.woff2",
        include_bytes!("../../front-end/fonts/OpenDyslexic-Bold.woff2"),
    ),
    (
        "fonts/OpenDyslexic-Bold-Italic.woff2",
        include_bytes!("../../front-end/fonts/OpenDyslexic-Bold-Italic.woff2"),
    ),
    (
        "fonts/OpenDyslexicMono-Regular.otf",
        include_bytes!("../../front-end/fonts/OpenDyslexicMono-Regular.otf"),
    ),
];

// A (file_name, file_contents) pair
pub(crate) static SOURCE_CODE_PRO: (&str, &[u8]) = (
    "fonts/source-code-pro-v11-all-charsets-500.woff2",
    include_bytes!("../../front-end/fonts/source-code-pro-v11-all-charsets-500.woff2"),
);
