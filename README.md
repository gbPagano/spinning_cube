# Spinning Cube

Perspective projection of a spinning cube, using using just ASCII characters.

---

video

## Instalation

```bash
cargo install spinning_cube
```

## Basic Usage

To display the cube in the terminal run the following command:

```bash
spinning_cube
```

By default the cube will be displayed in black and white, but it is also possible to display it in color! 

For that you can run the following command

```bash
spinning_cube -c
```

or

```bash
spinning_cube --colorful
```

To simulate perspective a distance value is used whose default value is 11. When changed your distortion is affected, try experimenting with values ​​like 3 and 4. 

To specify this distance use the following command

```bash
spinning_cube -d 4
```

or

```bash
spinning_cube --distance 4
```


