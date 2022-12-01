# Cerebellum

This language will use polish notation, so will be unique in being not stack based. There are only 3 types:

- Numbers, can be float or integer. Program will decide for you.
- Lists/Iterators, can be infinite or finite. Are lazy, will only be evaluated when needed.
- True, equivolent to 1 is most situations
- False/None, equivolent to 0 in most situations

Other types are represented as follows:

- Characters are just numbers
- Strings are just lists of characters
- Note: Many string functions will treat lists of characters as or. Thus `replaceChar("abc")` will actually replace on any `a`,
`b` OR `c`, while `replaceString("abc")` will only replace if the characters appear in a row.

This also means many, if not most string functions will work on arrays of other data too, including arrays of arrays.

All types are immutable, all operations make a copy. For example, using `setN` on a list will actually return a new list with the `n-th`
element replaced, leaving the old one unchanged.

All operations have fixed arity. Only the `Splat` and `SplatSingle` commands can override this.

## Iterator Commands (6)

These functions will implititly create a new function. No lambda needed. Note: Lists may be infinite.

* Map (Might be not neded, implicit?)
* Filter
* Reduce
* Scan
* Successors
* FixedPoint (Apply till unchanged)
* Takewhile
* Any
* All

## Iterator Util (9)

All these will create a new list.

* Chain (concat)
* ChainInfinite
* Flatten
* First
* Last
* Nth
* Delete Nth
* Replace Nth
* Slice
* Delete Range
* Replace Range
* Limit
* Chunks
* ChunksOverlapping
* Zip
* ZipSelf
* Enumerate
* Transpose
* Repeat
* Once
* Pair
* N-Sized (Takes a array of size N, returns a N-dimentional array with coordinates equal to the values in the array. Fill with X)
* Sum
* Union
* Intersection
* Sort
* SortReverse
* Sort by key
* Intersperse
* All Equal?
* FilterUnique
* Count
* Uniquefy
* Average
* Median
* Mode
* Local Maxima
* To Sparse
* From Sparse (untruth in vyxal)

## Python Itertools

* Combinations
* Permutations
* Product
* Nth Self Product
* Powerset
* Sublists

## Range

* Range
* InfiniteRange
* RangeInclusive
* RangeWithStep
* Reverse
* RotateLeft
* RotateRight

## Arithmetic 8

* `+`
* `-`
* `*`
* float division
* integer division
* `%`
* `&`
* `|`
* `^`
* Negate
* Reprical
* Power
* Log2
* Log10
* LogE
* LogN
* Sin
* Cos
* Tan
* asin
* atan
* Increment
* Decrement
* abs()
* Sqrt
* nth-root
* Factorial
* floor
* ceil
* round
* To degrees
* To radians
* 1 Count (binary)

## Logic (3)

* If
* Or (Short Cirquiting)
* And (Short Cirquiting)

## Variables(13)

* First of current function
* Second "
* Third
* First of parent scope
* "
* "
* First of nth scope
* Second "
* Third
* Input
* nth-command line argument
* Create Variable
* Read N-th variable
* Index in inner loop
* Index in parent loop
* Index in nth-parent loop
* Parent List
* Parent Parent List
* Nth-parent list

## Functions

* Create Function with 1 argument
* Create function with 2 arguments
* Create function with 3 arguments
* Call function with 1 argument
* Call function with 2 arguments
* Call function with 3 arguments
* Recurse
* Recurse File

## Constants(24)

* 1
* 2
* 3
* 4
* 5
* 6
* 7
* 8
* 9
* 10
* 11
* 12
* 13
* 14
* 15
* 16
* PI
* E
* Infinity
* 10
* 100
* 1000
* 256
* 512
* None

## Strings (4)

* SplitChar
* SplitString
* SplitWhitespace
* SplitWith
* SplitOnceChar
* SplitOnceString
* SplitOnceWith
* Upper
* Lower
* ContainsChar
* ContainsString
* ReplaceChar
* ReplaceString
* ReplaceOnceChar
* ReplaceOnceString
* ReplaceStringWith
* ReplaceCharWith
* StartsWithChar
* EndsWithChar
* Strip
* PadLeft
* PadLeftWith
* PadRight
* PadRightWith
* PadCenter
* PadCenterWith
* Join
* Matching Parenthesis

## Strings Constants

* Letters
* LowercaseLetters
* UppercaseLetters
* Symbols
* Printable
* Whitespace
* Empty String
* Digits


## IO

* Output as letters (Unicode code points)
* Output as numbers (repr)
* Print a space
* Print a line break
* Input
* Exit
* Current Time (Unix Time)
* Current Time (ISO)
* Current Time [year, month, day, hour, minute, second, millisecond]

## Logic

* Equals
* Less
* Greater
* LessOrEqual
* GreaterOrEqual
* Not
* Min
* Max
* Min2
* Max2
* IsNone
* InRange (Incl)
* InRange (Excl)

## Literals

* Number Literal (base64)
* String Literal (Raw)
* String Literal (base255)
* String Literal (ASCII printable Only)

## Parenthesis

* Break
* Continue

## Inspection

* Source Code
* Eval
* Filename
* Open
* Version
* Edit

# Linear Algebra

* Dot Product
* Cross Product
* Complex Multiply (Padded to power of 2, max 8)
* Matrix Multiply
* Diagonal
* Antidiagonal
* Inverse
* Quat to matrix

# Numbers to strings

* Stringify (repr's lists, decimals numbers)
* number to base 16
* number to base N
* string to number (base 10)
* String to number, n decimal places (base 10)
* stirng to number (base 16)
* string to number (base N)

# Primes

* Is Prime?
* Nth prime
* Factors
* Prime Factors
* GCD
* LCM

## Random

* Choice
* Number in range
* Float
* Condition
* Shuffle

## Special

* Splat (apply a array to multiple arguments of a function, overrides arity)
* SplatSingle (apply a single value to all arguments of a function)
* Get Arity
* Copy
* Deepcopy
* IsFinite: Return the list evaluated if it's finite. Otherwise, return None
* Discard: Evaluate a element but return None
