// scripts/script.js
document.getElementById("greeting-button").addEventListener("click", function() {
    const greetingInput = document.getElementById("greeting-input").value;
    const greetingOutput = document.getElementById("greeting-output");
    
    if (greetingInput) {
        greetingOutput.textContent = "You said: " + greetingInput;
    } else {
        greetingOutput.textContent = "Please enter a greeting!";
    }
});
