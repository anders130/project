pub struct Lang {
    pub name: &'static str,
    pub icon: &'static str,
    pub indicators: &'static [&'static str],
    pub exclude_if: &'static [&'static str],
}

pub const LANGS: &[Lang] = &[
    Lang {
        name: "Nix",
        icon: "\u{F313}",
        indicators: &["flake.nix", "default.nix"],
        exclude_if: &[],
    },
    Lang {
        name: "Rust",
        icon: "\u{E7A8}",
        indicators: &["Cargo.toml"],
        exclude_if: &[],
    },
    Lang {
        name: "TypeScript",
        icon: "\u{E628}",
        indicators: &["tsconfig.json"],
        exclude_if: &[],
    },
    Lang {
        name: "Node",
        icon: "\u{E718}",
        indicators: &["package.json"],
        exclude_if: &["tsconfig.json"],
    },
    Lang {
        name: "Python",
        icon: "\u{E606}",
        indicators: &["pyproject.toml", "requirements.txt", "setup.py"],
        exclude_if: &[],
    },
    Lang {
        name: "Go",
        icon: "\u{E724}",
        indicators: &["go.mod"],
        exclude_if: &[],
    },
    Lang {
        name: "Java",
        icon: "\u{E738}",
        indicators: &["pom.xml", "build.gradle"],
        exclude_if: &[],
    },
    Lang {
        name: "Ruby",
        icon: "\u{E23E}",
        indicators: &["Gemfile"],
        exclude_if: &[],
    },
    Lang {
        name: "PHP",
        icon: "\u{E608}",
        indicators: &["composer.json"],
        exclude_if: &[],
    },
    Lang {
        name: "Docker",
        icon: "\u{E7B0}",
        indicators: &["Dockerfile", "docker-compose.yml"],
        exclude_if: &[],
    },
];
