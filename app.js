
function clearScreen() {
    document.getElementById("result").value = "";
}
function backSpace(){
    document.getElementById("result").value = document.getElementById("result").value.slice(0, -1);
}
function pushToStack() {
    document.getElementById("result").value += " "
}
function display(value){
    document.getElementById("result").value += value;
}
function operator(value){
    display(value);
}

// Calculate Expression (Post-Fix) via Rust Import
function calculate(){
    const expr = document.getElementById("result");
    
    const result = 0;

    document.getElementById("result").value = result;
}
