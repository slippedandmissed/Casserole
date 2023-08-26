import { return_size } from "../../module/casserole/casserole.js";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");


export default {

    getScreenDimensions: () => {
        return return_size(
            canvas.width,
            canvas.height,
        );
    },

    setFillStyle: (fillStyle) => {
        ctx.fillStyle = fillStyle;
    },

    fillRect: (x, y, width, height) => {
        ctx.fillRect(x, y, width, height);
    }

};