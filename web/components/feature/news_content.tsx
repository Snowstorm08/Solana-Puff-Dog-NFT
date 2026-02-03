import { useMemo, useCallback } from "react";
import Image from "next/image";
import Link from "next/link";
import { NewsFeedUrl, NewsFeedStatus } from "graphql/generated/graphql";
import NewsHeader from "components/generic/news_header";
import { sharedByText } from "app/util";
import { timeSince } from "app/time";

interface NewsContentProps {
  newsFeedUrls: NewsFeedUrl[];
  newsFeedStatus?: NewsFeedStatus;
}

export default function NewsContent({
  newsFeedUrls,
  newsFeedStatus
}: NewsContentProps) {
  const lastUpdated = useMemo(() => {
    return newsFeedStatus?.completedAt
      ? timeSince(newsFeedStatus.completedAt)
      : "";
  }, [newsFeedStatus?.completedAt]);

  const renderNewsItem = useCallback(
    (newsFeedUrl: NewsFeedUrl) => {
      const {
        expandedUrlParsed,
        expandedUrlHost,
        previewImageThumbnailUrl,
        title,
        urlSlug
      } = newsFeedUrl;

      const externalUrl = expandedUrlParsed ?? "#";
      const imageSrc =
        previewImageThumbnailUrl || "/news_article_placeholder.png";

      return (
        <li
          key={urlSlug}
          className="grid grid-cols-12 gap-2 py-4 border-b last:border-b-0"
        >
          {/* Title & metadata */}
          <div className="col-span-10">
            <a
              href={externalUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="hover:underline"
            >
              <p className="text-base font-semibold">
                {title}
                {expandedUrlHost && (
                  <span className="ml-1 text-gray-600 font-normal">
                    ({expandedUrlHost})
                  </span>
                )}
              </p>
            </a>

            <p className="mt-1 text-sm text-gray-500">
              <Link
                href={{
                  pathname: "/news_feed/[url_slug]",
                  query: { url_slug: urlSlug }
                }}
                className="hover:underline"
              >
                {sharedByText(newsFeedUrl)}
              </Link>
            </p>
          </div>

          {/* Image preview */}
          <div className="col-span-2 flex items-start justify-center">
            <a
              href={externalUrl}
              target="_blank"
              rel="noopener noreferrer"
              aria-label={`Open article: ${title}`}
            >
              <Image
                src={imageSrc}
                alt={title || "News article preview"}
                width={80}
                height={80}
                className="rounded-md object-cover"
                sizes="(min-width: 1024px) 80px, 60px"
              />
            </a>
          </div>
        </li>
      );
    },
    []
  );

  return (
    <>
      <NewsHeader
        subtitle="Trending climate related articles shared by leading climate scientists, organizations, journalists and activists."
        lastUpdated={lastUpdated}
      />

      <div className="container mx-auto w-full px-4 md:max-w-3xl">
        <ul role="list">{newsFeedUrls.map(renderNewsItem)}</ul>
      </div>
    </>
  );
}
