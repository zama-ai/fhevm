// https://www.anthonygonzales.dev/blog/how-to-solve-match-media-is-not-a-function.html
import mediaQuery from 'css-mediaquery'

export function createMatchMedia(width) {
  window.matchMedia = query => ({
    matches: mediaQuery.match(query, {
      width,
    }),
    addListener: () => {},
    removeListener: () => {},
    onchange: () => {},
    media: query,
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => true,
  })
}
