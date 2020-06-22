package matchmaker

// Player represents metadata surrounding a player trying to enter a game
type Player struct {
	ELO               int
	Name              string
	AutoFillProtected bool
	Position          Position
}
