function submit() {
    const files = document.getElementById("files").files;

    const formData = new FormData();
    for (const file of files) {
        formData.append("file", file);
    }

    fetch("http://localhost:8080/upload", {
        method: 'POST',
        body: formData
    }).then(response => {
        if (response.ok) {
            alert("Successfully uploaded Files!\nRedirecting...")
            setTimeout(function () {
                window.location = "http://localhost:8080";
            }, 1000)
        } else {
            alert("Failed to upload files.")
        }
    }).catch((error) => {
        alert("Error: " + error);
    })
}

//Idk how it is broken :3c
function refresh() {
    setTimeout(function () {
        location.reload()
        refresh()
    }, 1000)
}