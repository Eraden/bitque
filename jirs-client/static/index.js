import("/jirs.js").then(async module => {
    // window.module = module;
    await module.default();
    module.render();
    document.querySelector('main').className = '';
    const spinner = document.querySelector('.spinner');
    spinner && spinner.remove();
});
