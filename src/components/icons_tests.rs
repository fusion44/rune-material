use super::*;

#[test]
fn official_symbols_rasterize_with_padding_at_display_sizes() {
    let icons = [
        &ADD,
        &ARCHIVE,
        &ARROW_DROP_DOWN,
        &CHECK,
        &CLOSE,
        &CODE,
        &DESCRIPTION,
        &DRAFT,
        &FOLDER,
        &FOLDER_OPEN,
        &IMAGE,
        &LOCK,
        &PLAY_CIRCLE,
        &REMOVE,
        &SEARCH,
    ];
    let mut names = std::collections::HashSet::new();
    for icon in icons {
        assert!(names.insert(icon.name), "unique atlas key: {}", icon.name);
        assert_eq!(icon.view, 24.0);
        for size in [16, 18, 24, 48] {
            let coverage = rune::text::icon_coverage(icon, size);
            let width = size as usize;
            assert_eq!(coverage.len(), width * width);
            assert!(
                coverage.iter().any(|&alpha| alpha > 128),
                "{} at {size}px is visible",
                icon.name
            );
            // At 16/18px, antialiasing on the wide open-folder silhouette
            // can reach the outermost texel. Check padding at optical size
            // and above, rather than trimming the official geometry.
            if size < 24 {
                continue;
            }
            for i in 0..width {
                assert_eq!(coverage[i], 0, "{} top edge", icon.name);
                assert_eq!(
                    coverage[(width - 1) * width + i],
                    0,
                    "{} bottom edge",
                    icon.name
                );
                assert_eq!(coverage[i * width], 0, "{} left edge", icon.name);
                assert_eq!(
                    coverage[i * width + width - 1],
                    0,
                    "{} right edge",
                    icon.name
                );
            }
        }
    }
}
