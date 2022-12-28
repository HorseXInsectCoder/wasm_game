// 引入wasm的js文件
import init, {hello} from '../pkg/wasm_game'

// 固定写法，必须先初始化
init().then(() => {
    hello("xx");
    console.log("init Ok");
})
