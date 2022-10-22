function clearScreen() {
    document.getElementById("result").value = "";
    document.getElementById("valueA").value = 0;
    document.getElementById("valueB").value = 0;
    document.getElementById("op").value = "";
}
function backSpace(){
    document.getElementById("result").value = document.getElementById("result").value.slice(0, -1);
}

function pushToStack() {
    if (document.getElementById("valueA").value == 0){
        document.getElementById("valueA").value = document.getElementById("result").value;
    } else if (document.getElementById("valueB").value == 0){
        document.getElementById("valueB").value = document.getElementById("result").value;
    } else {
        alert("Stack Full!");
    }
}

function display(value){
    document.getElementById("result").value += value;
}
function operator(value){
    document.getElementById("op").value = value;
}

// const rust = require("./calc/pkg/calc.js");
rust.then(m => m.greet()).catch(console.error)
function calculate(){
        let valueA = Number(document.getElementById("valueA").value);
        let valueB = Number(document.getElementById("valueB").value);
        let operation = document.getElementById("op").value;
        let result = "Error";

        if (isNaN(valueA) || isNaN(valueB)){
            // Number Error checking
            result = "NaN";
        } else if (Number.isSafeInteger(valueA) && Number.isSafeInteger(valueB)){
            // Integer Cases
            // result = rust.then(m => m.rust_calc_int(valueA, valueB, operation))
            // .catch(console.error);
            rust.then(m => m.greet("HelloWorld!")).catch(console.error);
        } else {
            // Float Cases
            result = rust.then(m => m.rust_calc_float(valueA, valueB, operation))
            .catch(console.error);
        }
        document.getElementById("result").value = result;
    }
