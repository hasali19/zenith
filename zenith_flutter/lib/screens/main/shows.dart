import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../api.dart';
import '../../widgets.dart';
import '../show_details.dart';

class ShowsScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return ShowsScreenState();
  }
}

class ShowsScreenState extends State<ShowsScreen> {
  Future<List<Show>> _shows;

  @override
  void initState() {
    super.initState();
    _shows = context.read<ApiClient>().getShows();
  }

  void _handleItemTap(Show show) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => ShowDetailsScreen(show),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: FutureBuilder<List<Show>>(
        future: _shows,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text('${snapshot.error}');
          }

          if (!snapshot.hasData) {
            return CircularProgressIndicator();
          }

          return PosterGrid(
            count: snapshot.data.length,
            poster: (i) => snapshot.data[i].poster,
            primary: (i) => snapshot.data[i].name,
            secondary: (i) => snapshot.data[i].startYear().toString(),
            onItemTap: (i) => _handleItemTap(snapshot.data[i]),
          );
        },
      ),
    );
  }
}
