# Fn, Struct, and Impl Algorithms

## Flowchart: `parse_fn_decl()`

```mermaid
graph TD
    Start((Begin fn)) --> Modifiers[Consume Modifiers]
    Modifiers --> AsyncCheck{peek Async}
    AsyncCheck -- Yes --> ConsumeAsync[is_async true] --> ExpectFn
    AsyncCheck -- No --> IsAsyncFalse[is_async false] --> ExpectFn

    ExpectFn --> AdvanceFn[expect Fn] --> AdvanceName[name expect Identifier]
    AdvanceName --> ParamLoop{peek Colon or Minus}

    ParamLoop -- No --> ParamId[p_name expect Identifier] --> ParamTypeCheck{peek Dot}
    ParamTypeCheck -- Yes --> ConsumeDot[advance Dot] --> ParseType[v_type parse_type_expr] --> PushParam
    ParamTypeCheck -- No --> TypeNone[v_type None] --> PushParam
    PushParam[push Parameter] --> ParamLoop

    ParamLoop -- Minus --> ConsumeMinus[advance Minus] --> RetType[rtype parse_type_expr] --> ExpectColon
    ParamLoop -- Colon --> ExpectColon[expect Colon]

    RetType --> ExpectColon
    ExpectColon --> ExpectSE[expect StatementEnd] --> ParseBody[body parse_block] --> End((Emit FnDecl))
```

## parse_fn_decl()

1. Build `modifiers = []` (check/consume `Pub, Priv, Static`).
2. `is_async = match_token(Async)`.
3. `expect(Fn)`, `name = expect(Identifier)`.
4. Loop `while !check(Colon) && !check(Minus)`:
   - `param_name = expect(Identifier)`.
   - `var_type = None`. If `match_token(Dot)`, `var_type = parse_type_expr()`.
   - Push parameter to `params`.
5. Return Type: `rtype = None`. If `match_token(Minus)`, `rtype = parse_type_expr()`.
6. Enter block: `expect(Colon)`, `expect(StatementEnd)`.
7. `body = parse_block()`. Return `Stmt::FnDecl`.

## Flowchart: `parse_struct_decl()`

```mermaid
graph TD
    Start((Begin struct)) --> Mods[Consume Modifiers] --> ConsKW[expect Struct] --> ConsName[name expect Identifier]

    ConsName --> CheckGen{peek LeftBracket}
    CheckGen -- Yes --> ConsLBr[advance LeftBracket] --> GenLoop{peek RightBracket}
    CheckGen -- No --> BlockIn

    GenLoop -- No --> ConsGenId[expect Identifier push params] --> CheckGenComma{match Comma}
    CheckGenComma -- Yes --> GenLoop
    CheckGenComma -- No --> GenLoop
    GenLoop -- Yes --> ConsRBr[advance RightBracket] --> BlockIn

    BlockIn --> ConsCol[expect Colon] --> ConsSE[expect StatementEnd] --> ConsInd[expect Indent] --> FieldLoop{peek Dedent}

    FieldLoop -- No --> CheckFieldSE{match StatementEnd}
    CheckFieldSE -- Yes --> FieldLoop
    CheckFieldSE -- No --> FieldName[f_name expect Identifier] --> ConsDot[expect Dot] --> ParseFType[type parse_type_expr] --> SaveField[push Field] --> FieldLoop

    FieldLoop -- Yes --> ExpectDedent[advance Dedent] --> End((Emit StructDecl))
```

## parse_struct_decl()

1. Retrieve `modifiers`.
2. `expect(Struct)`, `name = expect(Identifier)`.
3. Generics: `type_params = []`. If `match_token(LeftBracket)`:
   - Loop `while !check(RightBracket)`:
     - Push `expect(Identifier)`.
     - `match_token(Comma)`.
   - `expect(RightBracket)`.
4. `expect(Colon)`, `expect(StatementEnd)`.
5. Enter block: `expect(Indent)`. Setup `fields = []`.
6. Loop `while !check(Dedent)`:
   - If `match_token(StatementEnd)`, continue.
   - `field_name = expect(Identifier)`, `expect(Dot)`, `field_type = parse_type_expr()`. Push to `fields`.
7. `expect(Dedent)`, return `Stmt::StructDecl`.

## Flowchart: `parse_impl_decl()`

```mermaid
graph TD
    Start((Begin impl)) --> ExpImpl[expect Impl] --> TrgType[target parse_type_expr]
    TrgType --> ExpCol[expect Colon] --> ExpSE[expect StatementEnd] --> ExpInd[expect Indent]

    ExpInd --> LoopMethods{peek Dedent}

    LoopMethods -- No --> CheckSE{match StatementEnd}
    CheckSE -- Yes --> LoopMethods
    CheckSE -- No --> ParseFn[push parse_fn_decl] --> LoopMethods

    LoopMethods -- Yes --> AdvDed[advance Dedent] --> End((Emit ImplDecl))
```

## parse_impl_decl()

1. `expect(Impl)`.
2. `target = parse_type_expr()`.
3. Block entry: `expect(Colon)`, `expect(StatementEnd)`, `expect(Indent)`.
4. Setup `methods = []`.
5. Loop `while !check(Dedent)`:
   - If `match_token(StatementEnd)`, continue.
   - `methods.push(parse_fn_decl())`.
6. `expect(Dedent)`, return `Stmt::ImplDecl`.

## parse_block() helper

1. `expect(Indent)`. `stmts = []`.
2. Loop `while !check(Dedent) && !is_at_end()`:
   - If `match_token(StatementEnd)`, continue.
   - `stmts.push(parse_declaration())`.
3. `expect(Dedent)`. Return `stmts`.
