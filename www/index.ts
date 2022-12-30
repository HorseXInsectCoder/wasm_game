// 引入wasm的js文件
import init, { World, Direction } from '../pkg/wasm_game';
import { random } from "./utils/random";

const WORLD_WIDTH = 8;         // 格子个数
const REFRESH = 100;

// 固定写法，必须先初始化
init().then(wasm => {
    const CELL_SIZE = 20;       // 格子的宽度

    // 令蛇出现的位置随机
    // const snakeIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH)
    const snakeIndex = random(WORLD_WIDTH * WORLD_WIDTH);      // 这里由于Math.random()产生0到1之间的数，所以不会超出格子

    const world = World.new(WORLD_WIDTH, snakeIndex);
    const worldWidth = world.get_width();

    // 创建画布
    const canvas = <HTMLCanvasElement>document.getElementById("snake-world");
    const context = canvas.getContext("2d");

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;

    // 控制方向
    document.addEventListener("keydown", e => {
        switch (e.code) {
            case "ArrowUp":
                world.change_snake_direction(Direction.UP);
                break;
            case "ArrowDown":
                world.change_snake_direction(Direction.DOWN);
                break;
            case "ArrowLeft":
                world.change_snake_direction(Direction.LEFT);
                break;
            case "ArrowRight":
                world.change_snake_direction(Direction.RIGHT);
                break;
        }
    })

    function drawWorld() {
        context.beginPath();

        // 先画竖行
        for (let x = 0; x < worldWidth + 1; x++) {
            // 把点移动到每个格子的开始位
            context.moveTo(CELL_SIZE * x, 0);          // moveTo：把路径移动到画布中的指定点，不创建线条

            // 画线，CELL_SIZE * x 画一条竖线下来
            context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth);      // 添加一个新点，然后在画布中创建从该点到最后指定点的线条
        }

        for (let y = 0; y < worldWidth + 1; y++) {
            context.moveTo(0, CELL_SIZE * y);
            context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
        }

        context.stroke();
    }

    function drawSnake() {
        // 接收后端传来的指针
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,         // buffer
            world.snake_cells(),        // 指针
            world.snake_length()        // 数据长度
        );

        // cellIndex是位置，i是蛇身的第几个元素
        snakeCells.forEach((cellIndex, i) => {
            const row = Math.floor(cellIndex / worldWidth);
            const col = cellIndex % worldWidth;

            context.beginPath();
            // 给蛇头不同的颜色
            context.fillStyle = i === 0 ? '#787878' : '#000000';
            context.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
        })

        context.stroke();
    }

    function drawReward() {
        const reward_index = world.reward_cell();

        // 设snake_index是10，worldWidth是8，那么row = 1
        const row = Math.floor(reward_index / worldWidth);   // 取整，获取蛇头当然所在行
        // col = 2，所以蛇头最初出现的坐标是(1,2)，下标从0开始
        const col = reward_index % worldWidth;

        context.beginPath();
        // 给蛋不同的颜色
        context.fillStyle = '#FF0000';
        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );

        context.stroke();
    }

    function draw() {
        drawWorld();
        drawSnake();
        drawReward();
    }

    function run() {
        setTimeout(() => {
            // 清理画布
            context.clearRect(0, 0, canvas.width, canvas.height);
            world.update_snake();
            draw();

            requestAnimationFrame(run);
        }, REFRESH)
    }

    run();
})
