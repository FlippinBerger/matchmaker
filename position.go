package matchmaker

// Pos is an enum to represent the different positions
type Pos int

const (
	unknown Pos = iota
	// Top Pos
	Top
	// Jungle Pos
	Jungle
	// Middle Pos
	Middle
	// Bottom Pos
	Bottom
	// Support Pos
	Support
)

// Position houses player positional preferences
type Position struct {
	Fill   bool
	First  Pos
	Second Pos
}
