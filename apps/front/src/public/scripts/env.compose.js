// 
(function (window) {
  window.env = window.env || {};
  window['env']['VITE_BACK_HTTP_URL'] = 'http://localhost:3005/graphql';
  window['env']['VITE_BACK_WS_URL'] = 'ws://localhost:3005/graphql';
  window['env']['VITE_FLAG_INVITATIONS'] = false;
})(this);
