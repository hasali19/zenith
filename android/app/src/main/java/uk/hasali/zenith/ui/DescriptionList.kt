package uk.hasali.zenith.ui

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.Layout
import androidx.compose.ui.layout.Measurable
import androidx.compose.ui.layout.Placeable
import androidx.compose.ui.layout.layoutId
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

class DescriptionListScope {
    sealed interface Item {
        data class Entry(val name: String, val value: String) : Item
        data class Heading(val text: String) : Item
    }

    val items = mutableListOf<Item>()

    fun entry(name: String, value: String) {
        items.add(Item.Entry(name, value))
    }

    fun heading(text: String) {
        items.add(Item.Heading(text))
    }
}

private sealed interface DescriptionListLayoutItem<out T, out U, out V> {
    data class Entry<T, U>(val name: T, val value: U) : DescriptionListLayoutItem<T, U, Nothing>
    data class Heading<V>(val text: V) : DescriptionListLayoutItem<Nothing, Nothing, V>
}

@Composable
fun DescriptionList(modifier: Modifier = Modifier, content: DescriptionListScope.() -> Unit) {
    val colSpacing = with(LocalDensity.current) {
        16.dp.toPx().toInt()
    }

    val groupSpacing = with(LocalDensity.current) {
        8.dp.toPx().toInt()
    }

    val items = DescriptionListScope()
        .also(content)
        .items

    Layout(
        modifier = modifier,
        content = {
            for (item in items) {
                when (item) {
                    is DescriptionListScope.Item.Entry -> {
                        Text(
                            item.name,
                            style = MaterialTheme.typography.body2,
                            modifier = Modifier.layoutId(0),
                        )
                        Text(
                            item.value,
                            style = MaterialTheme.typography.body2,
                            modifier = Modifier.layoutId(0),
                        )
                    }

                    is DescriptionListScope.Item.Heading -> {
                        Text(
                            item.text,
                            style = MaterialTheme.typography.body2,
                            fontWeight = FontWeight.Bold,
                            modifier = Modifier.layoutId(1)
                        )
                    }
                }
            }
        },
    ) { measurables, constraints ->
        val firstPass = mutableListOf<DescriptionListLayoutItem<Placeable, Measurable, Placeable>>()
        var xOffset = 0

        var i = 0
        while (i < measurables.size) {
            when (measurables[i].layoutId as Int) {
                0 -> {
                    val name = measurables[i].measure(constraints)
                    if (name.width > xOffset) xOffset = name.width
                    firstPass.add(DescriptionListLayoutItem.Entry(name, measurables[i + 1]))
                    i += 2
                }

                1 -> {
                    firstPass.add(
                        DescriptionListLayoutItem.Heading(
                            measurables[i].measure(
                                constraints
                            )
                        )
                    )
                    i += 1
                }
            }
        }

        xOffset += colSpacing

        val placeables = firstPass.map {
            when (it) {
                is DescriptionListLayoutItem.Heading -> it
                is DescriptionListLayoutItem.Entry -> {
                    val name = it.name
                    val value =
                        it.value.measure(constraints.copy(maxWidth = constraints.maxWidth - xOffset))
                    DescriptionListLayoutItem.Entry(name, value)
                }
            }
        }

        var height = placeables.sumOf {
            when (it) {
                is DescriptionListLayoutItem.Entry -> maxOf(it.name.height, it.value.height)
                is DescriptionListLayoutItem.Heading -> it.text.height + groupSpacing
            }
        }

        if (placeables.firstOrNull() is DescriptionListLayoutItem.Heading) {
            height -= groupSpacing
        }

        layout(constraints.maxWidth, height) {
            var isFirst = false
            var yPosition = 0

            placeables.forEach {
                when (it) {
                    is DescriptionListLayoutItem.Heading -> {
                        if (!isFirst) {
                            yPosition += groupSpacing
                        }

                        it.text.placeRelative(x = 0, y = yPosition)
                        yPosition += it.text.height
                    }
                    is DescriptionListLayoutItem.Entry -> {
                        it.name.placeRelative(x = 0, y = yPosition)
                        it.value.placeRelative(x = xOffset, y = yPosition)
                        yPosition += maxOf(it.name.height, it.value.height)
                    }
                }

                isFirst = false
            }
        }
    }
}
