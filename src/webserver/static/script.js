const username = prompt('qual teu nome corno', 'anonimo');
const avatarUrl = prompt('link do avatar', 'https://google.com/favicon.ico');

const socket = new WebSocket('ws://localhost:3012');

const chat = document.getElementById('chat');

const md = markdownit();

socket.onmessage = (event) => {
    const message = JSON.parse(event.data);

    const messageDiv = document.createElement('div');
    messageDiv.classList.add('message');

    const avatar = document.createElement('img');
    avatar.classList.add('avatar');
    avatar.src = message.author.avatar_url;

    const wrapper = document.createElement('div');
    wrapper.classList.add('wrapper');

    messageDiv.appendChild(avatar);
    messageDiv.appendChild(wrapper);

    const username = document.createElement('span');
    username.classList.add('username');
    username.innerText = message.author.username;

    const content = document.createElement('span');
    content.innerHTML = md.render(message.content);

    wrapper.appendChild(username);
    wrapper.appendChild(content);

    chat.appendChild(messageDiv);

    chat.scrollTop = chat.scrollHeight;
}

const text = document.getElementById('text');

document
    .getElementById('message')
    .addEventListener('submit', (event) => {
    event.preventDefault();
    if (text.value) {
        socket.send(JSON.stringify({
            content: text.value,
            author: {
                username,
                avatar_url: avatarUrl
            }
        }))
        text.value = '';
    }
})
