package matchmaker

// Game represents a matched game
type Game struct {
	ELOSpread int
	RedTeam   Team
	BlueTeam  Team
}
