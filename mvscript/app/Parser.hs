{-# OPTIONS_GHC -Wno-unrecognised-pragmas #-}

{-# HLINT ignore "Use <$>" #-}
{-# HLINT ignore "Use $>" #-}
module Parser where

import Control.Applicative (liftA2)
import Control.Monad
import Data.Char
import Data.Text as T
import Text.ParserCombinators.Parsec
import Types

whitespace :: Parser ()
whitespace = void $ many $ oneOf " \n\t"

lexeme :: Parser a -> Parser a
lexeme p = p <* whitespace

parseStatement :: Parser Statement
parseStatement = (Decl <$> parseDeclaration) <|> (Expr <$> parseExpr) 

parseExpr :: Parser Expression
parseExpr = chainl1 parseTerm parseOperator

parseType :: Parser Expression
parseType = parseString <|> parseNum <|> parseBool

parseTerm :: Parser Expression
parseTerm = parseType <|> parseParens <|> parseVarIdentifier

parseDeclaration :: Parser Declaration
parseDeclaration = parseVarInitialization <|> parseVarDeclaration

parseNum :: Parser Expression
parseNum = Type <$> (try parseFloat <|> parseInt)

parseString :: Parser Expression
parseString = Type . String . T.pack <$> (char '"' *> many (noneOf "\"") <* lexeme (char '"'))

parseInt :: Parser Type
parseInt = Int . read <$> lexeme (many1 digit)

parseBool :: Parser Expression
parseBool = Type . Bool <$> (parseTrue <|> parseFalse)
    where
        parseTrue = lexeme $ True <$ string "true"
        parseFalse = lexeme $ False <$ string "false"

parseFloat :: Parser Type
parseFloat = Float . read <$> lexeme (liftA2 (++) (many1 digit) ((:) <$> char '.' <*> many digit))

parseVarIdentifier :: Parser Expression
parseVarIdentifier = VarIdentifier <$> identifier
  where
    identifier = lexeme ((:) <$> firstChar <*> many nonFirstChar)
    firstChar = letter <|> char '_'
    nonFirstChar = firstChar <|> digit

parseVarDeclaration :: Parser Declaration
parseVarDeclaration = 
    Variable <$> (lexeme (string "let ") *> parseVarIdentifier)
            <*> optionMaybe (lexeme (char ':') *> parseTypeName)
            <*> (lexeme (char ';') *> pure Nothing)

parseTypeName :: Parser TypeName
parseTypeName = parseIntTName <|> parseStringTName <|> parseFloatTName <|> parseBoolTName <|> parseVectorTName <|> parseMatrixTName <|> parsePointTName
    where
        parseIntTName = lexeme $ IntT <$ string "int"
        parseFloatTName = lexeme $ FloatT <$ string "float"
        parseBoolTName = lexeme $ BoolT <$ string "bool"
        parseStringTName = lexeme $ StringT <$ string "string"
        parsePointTName = lexeme $ PointT <$ string "point"
        parseVectorTName = lexeme $ VectorT <$ string "vector"
        parseMatrixTName = lexeme $ MatrixT <$ string "matrix"

    

parseVarInitialization :: Parser Declaration
parseVarInitialization = 
    Variable <$> (lexeme (string "let ") *> parseVarIdentifier)
            <*> optionMaybe (lexeme (char ':') *> parseTypeName)
            <*> (lexeme (char '=') *> parseExpr <* lexeme (char ';') >>= \expr -> return (Just expr))

parseParens :: Parser Expression
parseParens = Parentheses <$> (lexeme (char '(') *> parseExpr <* lexeme (char ')'))

parseOperator :: Parser (Expression -> Expression -> Expression)
parseOperator =
    lexeme $
        (char '+' *> pure (\lhs rhs -> Operation (Add lhs rhs)))
            <|> (char '-' *> pure (\lhs rhs -> Operation (Subtract lhs rhs)))
            <|> (char '*' *> pure (\lhs rhs -> Operation (Multiply lhs rhs)))
            <|> (char '/' *> pure (\lhs rhs -> Operation (Divide lhs rhs)))
            <|> (string "//" *> pure (\lhs rhs -> Operation (IntDivide lhs rhs)))
            <|> (char '>' *> pure (\lhs rhs -> Operation (GreaterThan lhs rhs)))
            <|> (char '<' *> pure (\lhs rhs -> Operation (LessThan lhs rhs)))
            <|> (string "==" *> pure (\lhs rhs -> Operation (Equals lhs rhs)))
            <|> (string "!=" *> pure (\lhs rhs -> Operation (NotEquals lhs rhs)))
            <|> (string ">=" *> pure (\lhs rhs -> Operation (GreaterThanEq lhs rhs)))
            <|> (string "<=" *> pure (\lhs rhs -> Operation (LessThanEq lhs rhs)))
            <|> (string "&&" *> pure (\lhs rhs -> Operation (And lhs rhs)))
            <|> (string "||" *> pure (\lhs rhs -> Operation (Or lhs rhs)))
            <|> (string "&" *> pure (\lhs rhs -> Operation (BitwiseAnd lhs rhs)))
            <|> (string "|" *> pure (\lhs rhs -> Operation (BitwiseOr lhs rhs)))
            <|> (string "^" *> pure (\lhs rhs -> Operation (BitwiseXor lhs rhs)))

-- parseFunctionDefinition
