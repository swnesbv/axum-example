    $('.item').addClass('run');
    $('.item_name').addClass('run');
    $('.item_text').addClass('run');
    // ..
    $(() => {
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item').removeClass('run');
            }, 0);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item').removeClass('active');
            }, 0);
        });
        // ..
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_name').removeClass('run');
            }, 0);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_name').removeClass('active');
            }, 0);
        });
        // ..
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_text').removeClass('run');
            }, 0);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_text').removeClass('active');
            }, 0);
        });
        // ..

        // .. // ..
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item').append($('.img:first'));
            }, 800);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_name').append($('.title:first'));
            }, 800);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_text').append($('.description:first'));
            }, 800);
        });
        // .. // ..

        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item').addClass('active');
            }, 800);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_name').addClass('active');
            }, 800);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.item_text').addClass('active');
            }, 800);
        });

        // ..
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.img:first-of-type').addClass('active');
            }, 800);
        });

        // ..
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.img:first-of-type').addClass('out');
            }, 0);
        });
        $('.sl_trigger').click(() => {
            setTimeout(() => {
                $('.img').removeClass('out');
            }, 800);
        });
    });
