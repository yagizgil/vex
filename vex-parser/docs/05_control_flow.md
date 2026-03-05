# Control Flow Algorithms

Target Nodes: `Stmt::If`, `Stmt::While`, `Stmt::For`, `Stmt::Match`

## Flowchart: `parse_if_stmt()`

```mermaid
graph TD
    Start((Begin if)) --> ExpectIf[expect If] --> ParseCond[cond parse_expression] --> ExpectBody[expect Colon StatementEnd] --> ParseThen[then_branch parse_block]

    ParseThen --> ElifLoop{match Elif}
    ElifLoop -- Yes --> ParseElifCond[elif_cond parse_expression] --> ExpectElifBody[expect Colon StatementEnd] --> ParseElifBlock[elif_body parse_block] --> PushElif[push elifs array] --> ElifLoop

    ElifLoop -- No --> ElseCheck{match Else}
    ElseCheck -- Yes --> ExpectElseBody[expect Colon StatementEnd] --> ParseElseBlock[else_branch parse_block] --> End((Emit If Node))
    ElseCheck -- No --> End
```

## parse_if_stmt()

1. `expect(If)`, `condition = parse_expression(0)`.
2. `expect(Colon)`, `expect(StatementEnd)`.
3. `then_branch = parse_block()`.
4. `elifs = []`. Loop `while match_token(Elif)`:
   - `elif_cond = parse_expression(0)`.
   - `expect(Colon)`, `expect(StatementEnd)`, `elif_body = parse_block()`.
   - Push `(elif_cond, elif_body)` to `elifs`.
5. `else_branch = None`. If `match_token(Else)`:
   - `expect(Colon)`, `expect(StatementEnd)`, `else_branch = Some(parse_block())`.
6. Return node.

## Flowchart: `parse_for_stmt()`

```mermaid
graph TD
    Start((Begin for)) --> ExpectFor[expect For] --> IdentLoop

    IdentLoop[expect Identifier push items array] --> CommaCheck{match Comma}

    CommaCheck -- Yes --> IdentLoop
    CommaCheck -- No --> ExpectIn[expect In] --> ParseIter[iterable parse_expression] --> ExpectColon[expect Colon StatementEnd] --> ParseBlock[body parse_block] --> End((Emit For Node))
```

## parse_for_stmt()

1. `expect(For)`. `items = []`.
2. Loop:
   - Push `expect(Identifier)`.
   - If `!match_token(Comma)`, break loop.
3. `expect(In)`, `iterable = parse_expression(0)`.
4. `expect(Colon)`, `expect(StatementEnd)`.
5. `body = parse_block()`. Return node.

## Flowchart: `parse_while_stmt()`

```mermaid
graph TD
    Start((Begin while)) --> ExpectWhile[expect While] --> ParseCond[cond parse_expression]
    ParseCond --> ExpectColSE[expect Colon StatementEnd] --> ParseBlock[body parse_block] --> End((Emit While Node))
```

## parse_while_stmt()

1. `expect(While)`, `condition = parse_expression(0)`.
2. `expect(Colon)`, `expect(StatementEnd)`.
3. `body = parse_block()`. Return node.

## Flowchart: `parse_match_stmt()`

```mermaid
graph TD
    Start((Begin match)) --> ExpMatch[expect Match] --> ParseCond[cond parse_expression] --> ExpColSE[expect Colon StatementEnd] --> ExpInd[expect Indent]

    ExpInd --> CaseLoop{peek Dedent}

    CaseLoop -- No --> CheckSE{match StatementEnd}
    CheckSE -- Yes --> CaseLoop
    CheckSE -- No --> ParsePattern[pattern parse_expression] --> ExpColSE2[expect Colon StatementEnd] --> ParseCaseBody[body parse_block] --> PushCase[push MatchCase] --> CaseLoop

    CaseLoop -- Yes --> ExpDed[advance Dedent] --> End((Emit Match Node))
```

## parse_match_stmt()

1. `expect(Match)`, `condition = parse_expression(0)`.
2. `expect(Colon)`, `expect(StatementEnd)`, `expect(Indent)`. Setup `cases = []`.
3. Loop `while !check(Dedent)`:
   - If `match_token(StatementEnd)`, continue.
   - `pattern = parse_expression(0)`.
   - `expect(Colon)`, `expect(StatementEnd)`.
   - `case_body = parse_block()`. Push `MatchCase { pattern, body: case_body }`.
4. `expect(Dedent)`, return node.
