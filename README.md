# Labyrinthine

A project made with Bevy, by Atlas-418.

## what even is it?
Labyrinthine is a game made by me (Atlas-418) for the [2024 FBLA (Future Buisness Leaders of America) Game design and simulation challenge](<https://connect.fbla.org/headquarters/files/High%20School%20Competitive%20Events%20Resources/Individual%20Guidelines/Presentation%20Events/Computer-Game-Simulation-Programming.pdf>). 
## What do I want to do with it? (A TODO list for me)
My goals:
- [ ] Procedural generation of the maze
- [ ] multiple paths, guided by riddles. Each path has it's own moral dilemma for following it
- [ ] figure out how to incorperate said moral dilemmas into the maze, and make it solevable via hints

And now the steps to achive those goals:
- [ ] Learn the Bevy API
- [ ] Get collisions and rendering
- [ ] Get a first person camera working
- [ ] move the first person camera around
- [ ] Figure out how to proceduraly generate a maze (check out [this blog](<https://professor-l.github.io/mazes/>))
- [ ] Apply the procedural maze

Things we need to make it through reigionals:
- [ ] Ability to move
- [ ] Moral dilemmas with multiple outcomes
- [ ] Allow the user to choose between the outcomes somehow
- [ ] Accessability options
- [ ] Title screen

And some random ideas I could implement: **Gimme judgement on these ideas.**
* Randomized colour pallete (use OKlab to generate?)
* Various different shader effects
  * incorperate into puzzles?
  * manifold garden-esque (gooch shading)
  * Chromatic abberation
  * Toon shading
  * B&W w/ lines only
  * ASCII render? (would be a big challenge lol)
* randomized texture on walls
  * noise map?
  * think naissance
* For the hints, consider how to make hints
  * option 1: have the maze build itself around a solution with a written riddles
  * option 2: write a riddle for each random maze
    * integrate chatGPT to write the riddles
    * have something with variables, somehow. let Jonah come up with it.
  * option 3: Just have a set maze. No random generation, just have a single map.
    * this is lame, and would loose us considerable points, no matter who the judge is.
