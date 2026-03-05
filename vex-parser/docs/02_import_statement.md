# Import Statement Algorithm

Target Node: `Stmt::Import { keyword: Token, path: Vec<Token> }`

## Flowchart Algorithm

```mermaid
graph TD
    Start((Begin)) --> ConsumeImport[advance Import] --> PathStart[Create new Path list]

    PathStart --> ExpId[expect Identifier push Path]
    ExpId --> CheckDot{peek is Dot}

    CheckDot -- Yes --> ConsumeDot[advance Dot] --> ExpId
    CheckDot -- No --> SaveAST[Push AST]

    SaveAST --> NextBranch{peek tok}

    NextBranch -- Identifier --> PathStart
    NextBranch -- Comma --> ConsumeComma[advance Comma] --> PathStart
    NextBranch -- StatementEnd --> ConsumeSE[advance StatementEnd] --> CheckIndent{peek Indent}
    NextBranch -- Dedent --> End((Return Vector))

    CheckIndent -- No --> End
    CheckIndent -- Yes --> ConsumeIndent[advance Indent] --> LoopBlock

    LoopBlock{peek tok2}
    LoopBlock -- StatementEnd --> ConsumeSE2[advance StatementEnd] --> LoopBlock
    LoopBlock -- Identifier --> PathStartBlock[Create Path]

    PathStartBlock --> ExpIdBlock[expect Identifier push Path]
    ExpIdBlock --> CheckDotBlock{peek is Dot}
    CheckDotBlock -- Yes --> ConsumeDotBlock[advance Dot] --> ExpIdBlock
    CheckDotBlock -- No --> SaveASTBlock[Push AST] --> LoopBlock

    LoopBlock -- Dedent --> ConsumeDedent[advance Dedent] --> End
```

## parse_import_stmt()

1. `keyword = advance()`.
2. Call `parse_single_import_path()`. Push returned `path` as `Stmt::Import`.
3. Loop `while check(TokenType::Identifier) || check(TokenType::Comma)`:
   - If `match_token(TokenType::Comma)`, consume it.
   - Call `parse_single_import_path()`. Push returned `path` as `Stmt::Import`.
4. Check for indented block:
   - If `peek() == StatementEnd` AND `peek_next() == Indent`:
     - `advance()` (Consume StatementEnd).
     - `advance()` (Consume Indent).
     - Loop `while !check(TokenType::Dedent)`:
       - If `match_token(TokenType::StatementEnd)`, continue.
       - Call `parse_single_import_path()`. Push as `Stmt::Import`.
     - `expect(TokenType::Dedent)`.

## parse_single_import_path()

1. Create `path = []`.
2. `expect(TokenType::Identifier)` -> Push to `path`.
3. Loop `while match_token(TokenType::Dot)`:
   - `expect(TokenType::Identifier)` -> Push to `path`.
4. Return `path` list.
