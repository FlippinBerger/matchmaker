package matchmaker

// Team represents a team ready to be put into a game
type Team struct {
	Players []Player
	Groups  []Group
	AvgELO  int
}
