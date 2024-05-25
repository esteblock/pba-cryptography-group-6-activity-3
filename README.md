# Group 6 Infinite deck poker using VRFs
Simple Poker.
0.- both players start with 100 points
1.- Players draw a card
2.- Playerws bet on their card.
3.- Players use a linear strategy on their card (0 to 100%/13) (If they have A means that they either win or draw)
4.- Players bet their strategy.
5.- Player with highest card wins and take the money, if there is a draw nothing happens.
6.- continues until one player does not have more momney


THe code is as follows.

1.- Players generate keypairs from hardcoded seeds
2.- they exchange public keys
3.- public reveal scheme to generate a random input for VRF
4.- each player computes a private VFR.

Here, we think that cards are ordered, so 52 mod 13 will be a card ranked from 1 to 13 (2,3.,4,5,6,7,8,9,10,J,Q,K,A)
5.- from the card they calculate the betting.
6.- they bet, singing the bet
7.- they verify using the key and signature
8,- they see who won
