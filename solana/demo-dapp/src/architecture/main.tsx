import mermaid from 'mermaid';
import { createRoot } from 'react-dom/client';

import { ArchitecturePage } from './ArchitecturePage';
import './styles.css';

mermaid.initialize({
  startOnLoad: false,
  securityLevel: 'strict',
  theme: 'base',
  themeVariables: {
    background: '#09090b',
    primaryColor: '#241a38',
    primaryBorderColor: '#a78bfa',
    primaryTextColor: '#f6f6f7',
    secondaryColor: '#102c24',
    secondaryBorderColor: '#14f195',
    secondaryTextColor: '#f6f6f7',
    tertiaryColor: '#1b1b20',
    tertiaryBorderColor: '#666672',
    tertiaryTextColor: '#f6f6f7',
    lineColor: '#a9a9b2',
    textColor: '#f6f6f7',
    actorBkg: '#1b1b20',
    actorBorder: '#666672',
    actorTextColor: '#f6f6f7',
    actorLineColor: '#666672',
    signalColor: '#d4d4d8',
    signalTextColor: '#f6f6f7',
    labelBoxBkgColor: '#1b1b20',
    labelBoxBorderColor: '#666672',
    labelTextColor: '#f6f6f7',
    loopTextColor: '#f6f6f7',
    noteBkgColor: '#241a38',
    noteBorderColor: '#a78bfa',
    noteTextColor: '#f6f6f7',
    fontFamily: 'Inter, ui-sans-serif, system-ui, sans-serif',
    fontSize: '18px',
  },
  flowchart: {
    curve: 'basis',
    htmlLabels: true,
    nodeSpacing: 54,
    rankSpacing: 64,
  },
  sequence: {
    actorMargin: 64,
    diagramMarginX: 32,
    diagramMarginY: 24,
    messageMargin: 36,
    mirrorActors: false,
    width: 190,
  },
});

createRoot(document.getElementById('root')!).render(<ArchitecturePage />);
