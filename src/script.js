varquery = document.getElementById('query');

function browse() {
    var input = query.value.trim();
    if (input) {
        if (input.startsWith('http://') || input.startsWith('https://')) {
            window.location.href = input;
        } else if (input.includes('.')) {
            window.location.href = 'http://' + input;
        } else {
            window.location.href = 'https://www.google.com/search?q=' + encodeURIComponent(input);
        }
    }
}

document.addEventListener('keypress', function(event) {
  if (event.key === 'Enter') {
    browse();
  }
});
