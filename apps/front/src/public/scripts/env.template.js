// 
(function (window) {
  window.env = window.env || {};
  window['env']['VITE_BACK_HTTP_URL'] = '${VITE_BACK_HTTP_URL}';
  window['env']['VITE_BACK_WS_URL'] = '${VITE_BACK_WS_URL}';
  window['env']['VITE_FLAG_INVITATIONS'] = '${VITE_FLAG_INVITATIONS}' === 'true';
})(this);