import { boundingBox } from "./katas/basic_bounding_box/solution.js";
export class Main {
    static call() {
        let data = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0]
        ];
        let res = boundingBox(data);
        console.log(`boundingBox. Data: ${data} ${res}`);
    }
}
Main.call();
//# sourceMappingURL=main.js.map