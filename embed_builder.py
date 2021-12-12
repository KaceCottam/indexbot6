from dataclasses import field, dataclass

import disnake

from util import scan, add, atop

MESSAGE_LIMIT = 512  # limit before adding new pages


@dataclass(eq=False)
class EmbedBuilder:
    title: str
    template: dict = field(default_factory=dict)
    pages: int = field(default=0, init=False)
    names: list[str] = field(default_factory=list, init=False)
    values: list[str] = field(default_factory=list, init=False)
    inlines: list[bool] = field(default_factory=list, init=False)

    def add_field(self, name: str, value: str, inline: bool = True):
        self.names.append(name)
        self.values.append(value)
        self.inlines.append(inline)

    def field_lengths(self) -> list[int]:
        """
        Find the lengths of each field we have
        :return: the partial sum of number of characters in the names and values combined.
        """
        return list(scan(map(atop(len, add), self.names, self.values)))

    def page_numbers(self) -> list[int]:
        """
        With this, we can find where to split the input fields and
        values into separate pages on every rising edge.
        :return: the page assignment for each field
        """
        return list(scan(
            # this gets the page for an item
            (field_length // MESSAGE_LIMIT for field_length in self.field_lengths()),
            # and if a page is skipped, it is no longer skipped
            lambda acc, x: acc + 1 if x - acc > 1 else x,
        ))

    def new_page_indices(self) -> list[int]:
        """
        Let us grab the unique values from page numbers
        and find the first index of that value in the page numbers
        :return: the field indices for which a new page will be made
        """
        page_numbers = self.page_numbers()
        return list(map(page_numbers.index, set(page_numbers)))

    def __str__(self):
        """
        Conversion to a string for embed builder
        results in the markdown representation of the full content.
        :return: markdown format
        """
        # we can define the page separator
        def page_separator(s, o):
            return f"\n\n{'-' * 10} {s}/{o} {'-' * 10}"

        # format url if there is one that isn't empty
        url = self.template.get("url")
        full_title = f"[{self.title}]({url})" if url else self.title

        # names are second heading after a newline
        names = (f"\n## {name}\n" for name in self.names)

        # add extra newline after each value
        values = (value for value in self.values)

        # page separators
        page_numbers = self.page_numbers()

        footer = self.template.get("footer") or ""
        if max(page_numbers) != 0:
            new_page_indices = self.new_page_indices()
            page_separators = (
                page_separator(page_numbers[k] + 1, page_numbers[-1] + 1)
                if k in new_page_indices
                else ""
                for k, _ in enumerate(names)
            )

            # join all the names and values with a newline
            body = "\n".join(map(add, map(add, names, values), page_separators))
        else:
            body = "\n".join(map(add, names, values))

        # return the markdown
        return f"# {full_title}\n{body}\n{footer}\n".strip()
        pass

    def build(self) -> list[disnake.Embed]:
        assert self.title is not None
        return list()
