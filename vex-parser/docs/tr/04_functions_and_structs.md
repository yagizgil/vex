# Fn, Struct ve Impl Blokları Algoritmaları

## Ayrıştırma Şeması: `parse_fn_decl()`

```mermaid
graph TD
    Start((Fonksiyon Basla)) --> Modifiers[Belirleyicileri Yut]
    Modifiers --> AsyncCheck{peek Async}
    AsyncCheck -- Evet --> ConsumeAsync[is_async true] --> ExpectFn
    AsyncCheck -- Hayir --> IsAsyncFalse[is_async false] --> ExpectFn

    ExpectFn --> AdvanceFn[expect Fn] --> AdvanceName[name expect Identifier]
    AdvanceName --> ParamLoop{peek Colon veya Minus}

    ParamLoop -- Hayir --> ParamId[p_name expect Identifier] --> ParamTypeCheck{peek Dot}
    ParamTypeCheck -- Evet --> ConsumeDot[advance Dot] --> ParseType[type parse_type_expr] --> PushParam
    ParamTypeCheck -- Hayir --> TypeNone[type None] --> PushParam
    PushParam[Array pushla] --> ParamLoop

    ParamLoop -- Minus --> ConsumeMinus[advance Minus] --> RetType[rtype parse_type_expr] --> ExpectColon
    ParamLoop -- Colon --> ExpectColon[expect Colon]

    RetType --> ExpectColon
    ExpectColon --> ExpectSE[expect StatementEnd] --> ParseBody[body parse_block] --> End((Dugumu Don))
```

## parse_fn_decl()

1. `modifiers = []` oluştur. `Pub`, `Priv`, `Static` tokenlerini döngü ile kontrol edip ekle.
2. `is_async = match_token(Async)` ile yakala.
3. `expect(Fn)` yut. `name = expect(Identifier)` yut.
4. Parametreleri (params) topla:
   - `params = []` oluştur.
   - Döngü: `while !check(Colon) && !check(Minus)`
     - `param_name = expect(Identifier)` yut.
     - `var_type = None` yap, eğer `match_token(Dot)` gelirse `var_type = parse_type_expr()`.
     - `params.push(Parameter { name, var_type })`.
5. Dönüş Tipi: `rtype = None`. Eğer `match_token(Minus)` ise `rtype = parse_type_expr()`.
6. Blok girişini yut: `expect(Colon)` ve `expect(StatementEnd)`.
7. `body = parse_block()`. `Stmt::FnDecl` objesini döndür.

## Ayrıştırma Şeması: `parse_struct_decl()`

```mermaid
graph TD
    Start((Struct Basla)) --> Mods[Belirleyicileri Yut] --> ConsKW[expect Struct] --> ConsName[name expect Identifier]

    ConsName --> CheckGen{peek LeftBracket}
    CheckGen -- Evet --> ConsLBr[advance LeftBracket] --> GenLoop{peek RightBracket}
    CheckGen -- Hayir --> BlockIn

    GenLoop -- Hayir --> ConsGenId[expect Identifier push] --> CheckGenComma{match Comma}
    CheckGenComma -- Evet --> GenLoop
    CheckGenComma -- Hayir --> GenLoop
    GenLoop -- Evet --> ConsRBr[advance RightBracket] --> BlockIn

    BlockIn --> ConsCol[expect Colon] --> ConsSE[expect StatementEnd] --> ConsInd[expect Indent] --> FieldLoop{peek Dedent}

    FieldLoop -- Hayir --> CheckFieldSE{match StatementEnd}
    CheckFieldSE -- Evet --> FieldLoop
    CheckFieldSE -- Hayir --> FieldName[f_name expect Identifier] --> ConsDot[expect Dot] --> ParseFType[type parse_type_expr] --> SaveField[Field push] --> FieldLoop

    FieldLoop -- Evet --> ExpectDedent[advance Dedent] --> End((Dugumu Don))
```

## parse_struct_decl()

1. `modifiers` topla.
2. `expect(Struct)` yut. `name = expect(Identifier)` yut.
3. Jenerikler: `type_params = []`. Eğer `match_token(LeftBracket)` (yani `[`) gelirse:
   - Döngü: `while !check(RightBracket)`
     - `type_params.push(expect(Identifier))`.
     - `match_token(Comma)`.
   - `expect(RightBracket)` (yani `]`) yut.
4. Blok girişini yut: `expect(Colon)` ve `expect(StatementEnd)`.
5. İçeri Gir: `expect(Indent)`. `fields = []` oluştur.
6. Döngü: `while !check(Dedent)`:
   - Eğer `match_token(StatementEnd)` ise atla (continue).
   - `field_name = expect(Identifier)` yut, `expect(Dot)`, `field_type = parse_type_expr()`. Listeye ekle.
7. `expect(Dedent)`, `Stmt::StructDecl` döndür.

## Ayrıştırma Şeması: `parse_impl_decl()`

```mermaid
graph TD
    Start((Impl Basla)) --> ExpImpl[expect Impl] --> TrgType[target parse_type_expr]
    TrgType --> ExpCol[expect Colon] --> ExpSE[expect StatementEnd] --> ExpInd[expect Indent]

    ExpInd --> LoopMethods{peek Dedent}

    LoopMethods -- Hayir --> CheckSE{match StatementEnd}
    CheckSE -- Evet --> LoopMethods
    CheckSE -- Hayir --> ParseFn[methods push parse_fn_decl] --> LoopMethods

    LoopMethods -- Evet --> AdvDed[advance Dedent] --> End((Dugumu Don))
```

## parse_impl_decl()

1. `expect(Impl)` yut.
2. `target = parse_type_expr()` (Bağlanacağı hedef tip).
3. `expect(Colon)`, `expect(StatementEnd)` ve `expect(Indent)` yut.
4. `methods = []` başlat.
5. Döngü: `while !check(Dedent)`:
   - Eğer `match_token(StatementEnd)` ise atla (continue).
   - `methods.push(parse_fn_decl())`.
6. `expect(Dedent)`, `Stmt::ImplDecl` döndür.

## Standart Blok Okuyucu Algoritması `parse_block()`

1. `expect(Indent)`. `stmts = []`.
2. Döngü: `while !check(Dedent) && !is_at_end()`
   - Eğer `match_token(StatementEnd)` ise continue.
   - `stmts.push(parse_declaration())`.
3. `expect(Dedent)`. `stmts` dizisini döndür.
