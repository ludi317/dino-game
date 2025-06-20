## Play the game [here](https://superb-cannoli-87eda6.netlify.app/)!
- Avoid cacti and pterodactyls
- Consume donuts

## Controls
- Space bar or up arrow ⬆️ to jump. On mobile, touch screen.
- Down arrow ⬇️ to duck.
- P to pause

## Design
- Parallax scrolling. Background layers scroll more slowly than foreground layers.
- Parametrically generated cacti.
- Progressively faster game speed.
- Animations:
  - Dino: Running, Ducking, Jumping, Dying
  - Cacti: Arms fall off if hit
  - Pterodactyl: Fall from sky if hit

## Demo
Note the gif is a bit choppy.

![Dino Gameplay](images/dino-game-2.gif)

Here is a screenshot of an earlier version of the game. Progress was made.

<img src="images/Early-screenshot.png" width="400"/>


## Origin Story
My kindergartner stumbled upon the [Chrome Dino game](https://en.wikipedia.org/wiki/Dinosaur_Game) one day when our internet went down. He was instantly hooked, and kept finding gaming websites to play the Dino game. 

I wasn’t thrilled about his new obsession. Site blockers and blacklists proved futile—he’d always find another site to play the game. Eventually, I won the battle with a DNS whitelist custom for his device.

When he begged to play again, I offered a compromise: he could play the game if we coded it ourselves.
This is how we came to create our own version of the Chrome Dino game, written in Rust using the Bevy engine.


## Build instructions for Web
Target wasm with javascript bindings

```bash
./build_web.sh
```
