// ipaddr is already declared when served

const form = document.querySelector('form');
const button = document.querySelector('#submit-button');
button.addEventListener('click', function(event) {
    event.preventDefault(); 
    const url = document.querySelector('#url-input').value.replace(/(^\w+:|^)\/\//, '');
    const encodedUrl = btoa(url);
    window.location.href = `/proxy/${encodedUrl}`;
});