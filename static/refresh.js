const baseurl = "http://licenses.haoliangsoftware.cn:3000/";
// const baseurl = "http://localhost:3000/";

let refresh = function () {
    var url = baseurl + 'admin/list';
    const request = new Request(url, {
        method: 'GET',
        mode: 'cors'
    });
    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return response.json();
            } else {
                throw new Error('Failed to fetch data');
            }
        })
        .then((data) => {
            console.log(data);
            let license_list = document.getElementById('license-list');

            if (data.length == 0) {
                license_list.innerHTML = '<p>暂无数据</p>';
            } else {
                license_list.innerHTML = '';
                const template = document.querySelector('#license-item-template');

                data.forEach((item) => {
                    console.log(item);
                    const clone = template.content.cloneNode(true);

                    let wrap = clone.querySelector('.license-item-wrap');
                    let key = clone.querySelector('.license-item-key');
                    let license = clone.querySelector('.license-item-license');
                    let update_ = clone.querySelector('.button-update');
                    let remove_ = clone.querySelector('.button-remove');

                    wrap.id = 'license-item-' + item.id;
                    console.log(wrap.id);

                    key.textContent = item.key;
                    license.value = item.license;
                    //license.readOnly = true;
                    update_.addEventListener('click', () => {
                        update(item.id);
                    });
                    remove_.addEventListener('click', () => {
                        remove(item.id);
                    });

                    license_list.appendChild(clone);
                });
            }
        });
};

let add = function () {
    let key = document.getElementById('add-key').value;
    let license = document.getElementById('add-license').value;
    let entry = JSON.stringify({
        key: key,
        license: license
    });

    var url = baseurl + 'admin/add';
    const request = new Request(url, {
        method: 'POST',
        mode: 'cors',
        body: entry,
        headers: {
            'Content-Type': 'application/json'
        }
    });
    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                console.log(response);
                refresh();
                return response.json();
            } else {
                throw new Error('Failed to fetch data');
            }
        });
};

let update = function(id) {
    console.log('license-item-' + id);
    let entry = document.getElementById('license-item-' + id);
    let key = entry.querySelector('.license-item-key').textContent;
    let license = entry.querySelector('.license-item-license').value;
    let json = JSON.stringify({
        id: id,
        key: key,
        license: license
    });

    var url = baseurl + 'admin/update';
    const request = new Request(url, {
        method: 'POST',
        mode: 'cors',
        body: json,
        headers: {
            'Content-Type': 'application/json'
        }
    });
    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                console.log(response);
                refresh();
            } else {
                throw new Error('Failed to fetch data');
            }
        });
};

let remove = function(id) {
    let entry = document.getElementById('license-item-' + id);
    let json = JSON.stringify({
        id: id,
    });

    var url = baseurl + 'admin/remove';
    const request = new Request(url, {
        method: 'POST',
        mode: 'cors',
        body: json,
        headers: {
            'Content-Type': 'application/json'
        }
    });
    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                console.log(response);
                refresh();
            } else {
                throw new Error('Failed to fetch data');
            }
        });
};

window.addEventListener('load', refresh, false);
