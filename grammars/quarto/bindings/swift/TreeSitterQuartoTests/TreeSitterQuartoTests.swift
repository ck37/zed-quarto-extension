import XCTest
import SwiftTreeSitter
import TreeSitterQuarto

final class TreeSitterQuartoTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_quarto())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Quarto grammar")
    }
}
