# Labyrinthine

A project made with Bevy, by Atlas-418.

## what even is it?
Labyrinthine is a game made by me (Atlas-418) for the [2024 FBLA (Future Buisness Leaders of America) Game design and simulation challenge](<https://connect.fbla.org/headquarters/files/High%20School%20Competitive%20Events%20Resources/Individual%20Guidelines/Presentation%20Events/Computer-Game-Simulation-Programming.pdf>). It will probably be bad, as it is the first game I have made, and on top of that, am not very good with rust. We're gonna try tho.
### Premise
Labyrinthine is going to be a short game, in which the player solves riddles to navigate a random maze in one of a couple (probably just two) paths. 
I plan on heavily focusing on rendering, and making sure the game runs smoothly, and I'll let jonah (my partner for this project) figure out storyline and whatnot. I'm the nerd. he's a nerd too, but this time, he gets to do creative stuff.

## What do I want to do with it? (A TODO list for me)
My goals:
- [ ] Procedural generation of the maze
- [ ] multiple paths, guided by riddles. Each path has it's own moral dilemma for following it
- [ ] figure out how to incorperate said moral dilemmas into the maze, and make it solevable via hints

And now the steps to achive those goals:
- [X] Get collisions and rendering
- [X] Get a first person camera working
- [X] move the first person camera around
- [ ] Figure out how to proceduraly generate a maze (check out [this blog](<https://professor-l.github.io/mazes/>), will most likely use Hunt and Kill)
- [X] Generate a 3D world with the procedural maze

Things we need to make it through reigionals:
- [X] Ability to move
- [ ] Moral dilemmas with multiple outcomes
- [ ] Allow the user to choose between the outcomes somehow
- [ ] Accessability options (?)
- [ ] Title screen / Introduction to game

And some random ideas I could implement: **Gimme judgement on these ideas.**
* For the hints, consider how to make hints
  * option 1: write a riddle for each random maze
    * choose a set number of junctions along solution paths, apply question to each junction.
  * option 2: Just have a set maze. No random generation, just have a single map.
    * this is lame.

# Documentation