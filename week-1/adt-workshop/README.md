# Introduction to Algebraic Datatypes

ADT's are a fundamental concept that are especially important for learning
Haskell. Before we can understand other more complex features in the language,
it is important to have an understanding of ADT's.

### Product Types

```hs
data Position
  = Position Int Int
  deriving (Show)
```

This datatype declaration creates a type `Position` *and* a constructor
for `Position`. At this point we could create a position using this:

```hs
$ >>> Position 3 5
```

A constructor is:
*  atomic
*  like a named tuple
*  *has* to be used to construct a value of the type

NOTE: Values and types have separate namespaces in Haskell, which is neat!

### Implicit Signatures

Every datatype declaration introduces implicit values with implicit types.
In the example above, the constructor would be of the following type:

```hs
Position :: Int -> Int -> Position
```

Another example of a helper function that might be defined with this type
would include

```hs
mkPosition :: Int -> Position
mkPosition n = Position n n
```

NOTE: It can be helpful to think of constructors as named tuples.

### Selectors

How would be give names/labels to the fields?

```hs
data Position2 =
  Position2 {
    posX :: Int,
    posY :: Int
  }
  deriving (Show)
```

### Deconstruction of Product Types

We might then want to inspect and operate on the fields of an object.
Haskell offers pattern matching!

```hs
isValid :: Position -> Bool
isValid (Position x y) =
  x >= 1 && x <= 8 &&
  y >= 1 && y <= 8
```

This would be an example to check if the position is a valid position on a
chess board. Note that we can destructure the position into two separate
variables, using pattern matching!

### Sum Types

Sum Types! These are extremely cool. We can define a type that contains
constructors. Similar to the previous example, we do have implicit signatures
here. Note that one of the constructors _has_ to be used to construct a
value of the sum type.

```hs
data Color
  = White
  | Black
  deriving (Show)
```

How would we deconstruct this?

```hs
isBlack :: Color -> Bool
isBlack = case color of
  Black -> True
  White -> False
```

### Sum of Products

Every datatype is a combination of sum types and product types.

```hs
data Piece
  = Pawn Position Color
  | Rook Position Color
  | Knight Position Color
  | Queen Position Color QueenOrigin
  deriving (Show)

data QueenOrigin
  = Original
  | Created
  deriving (Show)
```

What about deconstructing the pieces to calculate some of the moves that might
be available to a piece?

```hs
allowedMoves :: Piece -> [Position]
allowedMoves piece = case piece of
  Pawn (Position x y) color -> case color of
    White -> [Position x (y + 1)]
    Black -> [Position x (y - 1)]
  -> error "NYI"
```

### Misc.

Haskell in general does not have syntax for thinking about the memory
management of a program. This should, in general, be thought of as the
concerns of the compiler, rather than the programmer.

Don't model what you don't need!

Unused constructors and fields ar4e dead code too!

Model your domain as closely as possible.

In general, when refactoring, the experience is that one should refactor types,
and then work through the value related compiler errors. Once types are
declared are correct, the rest of the refactoring experience in general is
fairly smooth.

