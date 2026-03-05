# Enums, Macros, and Miscellaneous Statements

Target Nodes: `Stmt::EnumDecl`, `Stmt::MacroDecl`, `Stmt::DefineDecl`, `Stmt::Return`, `Stmt::Break`, `Stmt::Continue`

## Flowchart: `parse_enum_decl()`

```mermaid
graph TD
    Start((Begin enum)) --> Mods[Consume Modifiers] --> ConsKW[expect Enum] --> ConsName[name expect Identifier]

    ConsName --> CheckGen{peek LeftBracket}
    CheckGen -- Yes --> ConsLBr[advance LeftBracket] --> GenLoop{peek RightBracket}
    CheckGen -- No --> BlockIn

    GenLoop -- No --> ConsGenId[expect Identifier push params] --> CheckGenComma{match Comma}
    CheckGenComma -- Yes --> GenLoop
    CheckGenComma -- No --> GenLoop
    GenLoop -- Yes --> ConsRBr[advance RightBracket] --> BlockIn

    BlockIn --> ConsCol[expect Colon] --> ConsSE[expect StatementEnd] --> ConsInd[expect Indent] --> VariantLoop{peek Dedent}

    VariantLoop -- No --> CheckVarSE{match StatementEnd}
    CheckVarSE -- Yes --> VariantLoop
    CheckVarSE -- No --> VarName[variant expect Identifier] --> SaveVariant[push variant] --> CheckComma{match Comma}
    CheckComma -- Yes --> VariantLoop
    CheckComma -- No --> VariantLoop

    VariantLoop -- Yes --> ExpectDedent[advance Dedent] --> End((Emit EnumDecl))
```

## parse_enum_decl()

1. Retrieve `modifiers` (from routing lookahead).
2. `expect(Enum)`, `name = expect(Identifier)`.
3. Generics: `type_params = []`. If `match_token(LeftBracket)`:
   - Loop `while !check(RightBracket)`:
     - `type_params.push(expect(Identifier))`.
     - `match_token(Comma)`.
   - `expect(RightBracket)`.
4. Block Entry: `expect(Colon)`, `expect(StatementEnd)`, `expect(Indent)`.
5. Setup `variants = []`.
6. Loop `while !check(Dedent)`:
   - If `match_token(StatementEnd)`, continue.
   - `variants.push(expect(Identifier))`.
   - `match_token(Comma)` (allow optional commas between variants).
7. `expect(Dedent)`, return `Stmt::EnumDecl`.

---

## parse_define_decl()

_(Defines act as lightweight consts, e.g., `define PI 3.14`)_

1. `expect(Define)`.
2. `name = expect(Identifier)`.
3. `value = parse_expression(0)`.
4. `expect(StatementEnd)`.
5. Return `Stmt::DefineDecl`.

---

## parse_macro_decl()

_(Macros act as code-generation blocks, e.g., `macro loop count:`)_

1. `expect(Macro)`.
2. `name = expect(Identifier)`.
3. Setup `params = []`. Loop `while !check(Colon)`:
   - `params.push(expect(Identifier))`.
4. `expect(Colon)`, `expect(StatementEnd)`.
5. `body = parse_block()`.
6. Return `Stmt::MacroDecl`.

---

## Flowchart: Jump Statements

```mermaid
graph TD
    Start((Begin stmt)) --> Keyword{peek token}

    Keyword -- Return --> AdvRet[keyword advance] --> RetCheckSE{peek StatementEnd}
    RetCheckSE -- Yes --> RetNone[value None] --> RetEnd[expect SE] --> EmitRet((Emit Return Node))
    RetCheckSE -- No --> RetVal[value parse_expression] --> RetEnd

    Keyword -- Break --> AdvBrk[keyword advance] --> BrkSE[expect SE] --> EmitBrk((Emit Break Node))

    Keyword -- Continue --> AdvCont[keyword advance] --> ContSE[expect SE] --> EmitCont((Emit Continue Node))
```

## Statement Parses routines

### parse_return_stmt()

1. `keyword = advance()`.
2. If `!check(StatementEnd)` -> `value = Some(parse_expression(0))`.
3. Else -> `value = None`.
4. `expect(StatementEnd)`. Return `Stmt::Return`.

### parse_break_stmt()

1. `keyword = advance()`.
2. `expect(StatementEnd)`. Return `Stmt::Break`.

### parse_continue_stmt()

1. `keyword = advance()`.
2. `expect(StatementEnd)`. Return `Stmt::Continue`.
