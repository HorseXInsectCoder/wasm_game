// 引入wasm的js文件
import init, { World } from '../pkg/wasm_game'

const GRID_NUMBER = 8;         // 格子个数
const REFRESH = 500;

// 固定写法，必须先初始化
init().then(() => {
    const CELL_SIZE = 20;       // 格子的宽度

    const world = World.new(GRID_NUMBER);
    const worldWidth = world.get_width();

    // 创建画布
    const canvas = <HTMLCanvasElement>document.getElementById("snake-world");
    const context = canvas.getContext("2d");

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;

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
        const snake_index = world.snake_head_index();

        // 设snake_index是10，worldWidth是8，那么row = 1
        const row = Math.floor(snake_index / worldWidth);   // 取整，获取蛇头当然所在行
        // col = 2，所以蛇头最初出现的坐标是(1,2)，下标从0开始
        const col = snake_index % worldWidth;

        context.beginPath();
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
