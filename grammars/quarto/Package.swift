// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterQuarto",
    products: [
        .library(name: "TreeSitterQuarto", targets: ["TreeSitterQuarto"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterQuarto",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                // NOTE: if your language has an external scanner, add it here.
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterQuartoTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterQuarto",
            ],
            path: "bindings/swift/TreeSitterQuartoTests"
        )
    ],
    cLanguageStandard: .c11
)
