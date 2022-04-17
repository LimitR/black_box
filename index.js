async function get(){
    let request = await fetch("http://127.0.0.1:8000/dir");
    let arr = await request.json();
    document.querySelector(".list").innerHTML = await arr.dir.map(el => `<p><button class=${el.replace(".mp4", "")}>${el}</button></p>`);
}

async function btn(){
    document.querySelectorAll('.list')[0].addEventListener('click', element => {
        document.querySelector(".player").innerHTML = `<video  controls>
                                                         <source src="http://127.0.0.1:8000/video/${element.target.innerText}" type="video/mp4">
                                                       </video>`
    });
}
async function start(){
    await get();
    await btn();
}

start();