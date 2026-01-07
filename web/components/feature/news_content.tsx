import { useMemo } from "react";
import { NewsFeedUrl, NewsFeedStatus } from "graphql/generated/graphql";
import Link from "next/link";
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
    if (!newsFeedStatus?.completedAt) return "";
    return timeSince(newsFeedStatus.completedAt);
  }, [newsFeedStatus?.completedAt]);

  return (
    <>
      <NewsHeader
        subtitle="Trending climate related articles shared by leading climate scientists, organizations, journalists and activists."
        lastUpdated={lastUpdated}
      />

      <div className="container w-full px-4 mx-auto md:max-w-3xl">
        <ul>
          {newsFeedUrls.map((newsFeedUrl) => {
            const {
              expandedUrlParsed,
              expandedUrlHost,
              previewImageThumbnailUrl,
              title,
              urlSlug
            } = newsFeedUrl;

            return (
              <li
                key={expandedUrlParsed || urlSlug}
                className="grid grid-cols-12 my-4 gap-2"
              >
                {/* Title & shares */}
                <div className="col-span-10">
                  <a
                    href={expandedUrlParsed}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:underline"
                  >
                    <p className="text-base">
                      <strong>{title}</strong>{" "}
                      <span className="text-gray-600">
                        ({expandedUrlHost})
                      </span>
                    </p>
                  </a>

                  <p className="mt-1 text-base text-gray-500">
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
                    href={expandedUrlParsed}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <img
                      src={
                        previewImageThumbnailUrl ||
                        "/news_article_placeholder.png"
                      }
                      alt={title || "News article preview"}
                      className="h-15 w-15 rounded lg:h-20 lg:w-20 lg:rounded-md object-cover"
                    />
                  </a>
                </div>
              </li>
            );
          })}
        </ul>
      </div>
    </>
  );
}
