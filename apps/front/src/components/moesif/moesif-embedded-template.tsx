type Props = {
  embedTemplateUrls: string[];
};

export default function MoesifEmbeddedTemplate({ embedTemplateUrls }: Props) {
  return (
    <div className="dashboards page-layout__focus">
      <div className="dashboards-container">
        {embedTemplateUrls?.map((url, index) => (
          <iframe
            key={url}
            title={`Moesif Dash ${index}`}
            id={url}
            src={url}
            name="preview-frame"
          />
        ))}
      </div>
    </div>
  );
}
