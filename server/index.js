import { TextEditor } from "rust";

// Get the constant HTML elements
const textarea = document.getElementById("editor");
const div = document.getElementById("lines");

// Create a new Text Editor
const editor = TextEditor.new("java");

// Caret position
var xCaretPos = 0;
var yCaretPos = 0;

/**
 * Sets the caret on input
 * @param {number} xValueChange - Value to change the x position of the caret
 * @param {number} yValueChange - Value to change the y position of the caret
 * @param {boolean} resetX - If the x position should be reset to 0
 */
function setCaretPos(xValueChange, yValueChange, resetX) {
    if ((xCaretPos + xValueChange) < 0) {
        // Do not move over if the value would make it a negative value
        return;
    }

    if ((yCaretPos + yValueChange) < 0) {
        // Do not move over if the value would make it a negative value
        return;
    }

    if (resetX) {
        // Reset x position to 0
        xCaretPos = 0;
    }
    else {
        // Move caret over
        xCaretPos += xValueChange;
    }

    // Move caret over
    yCaretPos += yValueChange;

    // Grab focus of the textarea element and set the caret to the position
    textarea.focus();
    textarea.setSelectionRange(xCaretPos, xCaretPos);

    // Update the textarea to the new position
    textarea.style = "top: " + yCaretPos + "px; left: " + xCaretPos + "px;"

    // Grab focus and clear the input
    textarea.focus();
    textarea.value = '';
}

textarea.addEventListener("input", function(e) {
    // On input, handle event
    if (e.data == null) {
        editor.get_input(e.inputType, "");
        setCaretPos(0, 18, true);
    }
    else {
        editor.get_input(e.inputType, e.data);

        // Move caret over
        setCaretPos(8, 0, false);
    }
    div.innerHTML = editor.render();
})

textarea.addEventListener("keyup", function(e) {
    // Handle backspace and delete, since they are not processed as input events
    if (e.key == "Backspace") {
        editor.get_input("deleteContentBackward", "");
        setCaretPos(-8, 0, false);
        div.innerHTML = editor.render();
    }
    else if (e.key == "Delete") {
        // TODO
        editor.get_input("deleteContentForward", "");
        div.innerHTML = editor.render();
    }
})

div.addEventListener("click", function(e) {
    var xPosition = e.clientX;
    var yPosition = e.clientY;
    textarea.style = "top: " + yPosition + "px; left: " + xPosition + "px;"
})