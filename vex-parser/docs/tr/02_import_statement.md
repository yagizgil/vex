# Import İfadesi Algoritması

Hedef Düğüm: `Stmt::Import { keyword: Token, path: Vec<Token> }`

## Ayrıştırma Şeması (Flowchart)

```mermaid
graph TD
    Start((Basla)) --> Yut[advance Import] --> PathStart[Yeni Path Listesi]

    PathStart --> ExpId[expect Identifier Path at]
    ExpId --> CheckDot{peek Nokta}

    CheckDot -- Evet --> ConsumeDot[advance Dot] --> ExpId
    CheckDot -- Hayir --> SaveAST[Listeye pushla]

    SaveAST --> NextBranch{Sıradaki Token}

    NextBranch -- Identifier --> PathStart
    NextBranch -- Comma --> ConsumeComma[advance Comma] --> PathStart
    NextBranch -- StatementEnd --> ConsumeSE[advance StatementEnd] --> CheckIndent{peek Indent}
    NextBranch -- Dedent --> End((Vektor Don))

    CheckIndent -- Hayir --> End
    CheckIndent -- Evet --> ConsumeIndent[advance Indent] --> LoopBlock

    LoopBlock{Siradaki}
    LoopBlock -- StatementEnd --> ConsumeSE2[advance StatementEnd] --> LoopBlock
    LoopBlock -- Identifier --> PathStartBlock[Yeni Path Listesi]

    PathStartBlock --> ExpIdBlock[expect Identifier Pathe at]
    ExpIdBlock --> CheckDotBlock{peek Nokta}
    CheckDotBlock -- Evet --> ConsumeDotBlock[advance Dot] --> ExpIdBlock
    CheckDotBlock -- Hayir --> SaveASTBlock[Listeye at] --> LoopBlock

    LoopBlock -- Dedent --> ConsumeDedent[advance Dedent] --> End
```

## parse_import_stmt()

1. `keyword = advance()` (Import tokenini yut).
2. `parse_single_import_path()` çağır. Dönen `path` objesini `Stmt::Import` olarak ekle.
3. Alt satıra inene kadar topla:
   - Döngü: `while check(TokenType::Identifier) || check(TokenType::Comma)`
     - Eğer `match_token(TokenType::Comma)` ise yut.
     - `parse_single_import_path()` çağır ve yeni `Stmt::Import` ekle.
4. Alt satır blok kontrolü (Indent):
   - Eğer `peek() == StatementEnd` VE `peek_next() == Indent` ise:
     - `advance()` (StatementEnd yut), `advance()` (Indent yut).
     - Döngü: `while !check(TokenType::Dedent)`
       - Eğer `match_token(TokenType::StatementEnd)` ise atla (continue).
       - `parse_single_import_path()` çağır ve listeye ekle.
     - `expect(TokenType::Dedent)`.
5. Bitti.

## parse_single_import_path()

1. Boş `path = []` oluştur.
2. `expect(TokenType::Identifier)` çağır ve `path` içine pushla.
3. Döngü: `while match_token(TokenType::Dot)`
   - `expect(TokenType::Identifier)` çağır ve pushla.
4. `path` dizisini döndür.
