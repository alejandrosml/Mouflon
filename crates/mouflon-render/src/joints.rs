//! Les articulations : l'encolure entre le corps et la tête, les pattes en
//! deux segments (le carpe des antérieurs, le jarret des postérieurs).
//!
//! Le même calcul sert aux dessins vectoriels ([`crate::animate`]) et aux
//! squelettes peints ([`crate::skeleton`]) : chacun donne ses points au repos
//! (dans le repère de son dessin) et où en est le corps ; on lui rend où
//! mettre chaque segment.
//!
//! Le moteur part de ce qu'il faisait d'une pièce (la tête qui tourne et se
//! déplace, le sabot qui suit son chemin), puis le répartit entre les
//! articulations par une petite cinématique inverse à deux os : pour brouter,
//! le museau va au sol devant les antérieurs, et c'est l'encolure qui descend ;
//! au pas, le sabot reste posé et le genou se plie de lui-même quand la patte
//! se raccourcit. Voir `docs/animation-animale.md`.

use std::f64::consts::{FRAC_PI_2, PI};

use peniko::kurbo::{Affine, Point, Rect, Vec2};

use crate::animate::{Gait, about, ease, lowest_point, rotate_about, stepping};
use crate::gesture::{Gesturing, leg_slot};

/// L'angle `a` ramené entre -π et π.
fn wrap(a: f64) -> f64 {
    (a + PI).rem_euclid(2.0 * PI) - PI
}

fn angle(v: Vec2) -> f64 {
    v.y.atan2(v.x)
}

fn rotate(v: Vec2, a: f64) -> Vec2 {
    let (s, c) = a.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

/// Étire de `s` le long de la direction `dir` (radians), autour de `p`.
pub(crate) fn stretch_along(p: Point, dir: f64, s: f64) -> Affine {
    about(
        p,
        Affine::rotate(dir) * Affine::scale_non_uniform(s, 1.0) * Affine::rotate(-dir),
    )
}

/// Une chaîne de deux segments au repos : la racine (épaule, hanche, base
/// de l'encolure), l'articulation (carpe, jarret, nuque) et le bout (sabot,
/// museau). Ses longueurs et ses angles au repos sont calculés une fois.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Chain {
    pub root: Point,
    pub joint: Point,
    pub tip: Point,
    /// Longueurs des deux segments, de la racine au bout au repos.
    l1: f64,
    l2: f64,
    rest_len: f64,
    /// Direction du premier segment, pli au repos, direction de la racine au
    /// bout (son cosinus et son sinus).
    first_rest: f64,
    bend_rest: f64,
    axis: (f64, f64),
}

impl Chain {
    pub(crate) fn new(root: Point, joint: Point, tip: Point) -> Self {
        let first_rest = angle(joint - root);
        let axis = angle(tip - root);
        Self {
            root,
            joint,
            tip,
            l1: (joint - root).hypot().max(1e-6),
            l2: (tip - joint).hypot().max(1e-6),
            rest_len: (tip - root).hypot().max(1e-6),
            first_rest,
            bend_rest: wrap(angle(tip - joint) - first_rest),
            axis: (axis.cos(), axis.sin()),
        }
    }
}

/// La solution : de combien tourne le premier segment (autour de la
/// racine), de combien le second tourne en plus du premier (autour de
/// l'articulation), et de combien le tout s'étire.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Bend {
    pub first: f64,
    pub second: f64,
    pub stretch: f64,
}

impl Bend {
    pub(crate) fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            first: self.first + (other.first - self.first) * t,
            second: self.second + (other.second - self.second) * t,
            stretch: self.stretch + (other.stretch - self.stretch) * t,
        }
    }
}

impl Chain {
    /// Le pli au repos (le second segment par rapport au premier).
    fn rest_bend(&self) -> f64 {
        self.bend_rest
    }

    /// Le côté où l'articulation se plie : celui du dessin s'il est plié,
    /// sinon `natural` (1 : le second segment tourne dans le sens des
    /// aiguilles d'une montre, comme le canon d'un antérieur qui part en
    /// arrière ; -1 : l'inverse, comme celui d'un postérieur).
    fn side(&self, natural: f64) -> f64 {
        let rest = self.rest_bend();
        if rest.abs() > 0.02 {
            rest.signum()
        } else {
            natural
        }
    }

    /// La cinématique inverse : la racine posée en `root`, le bout en
    /// `target`. `fold` plie l'articulation au-delà de ce qu'il faut (le
    /// bout se rapproche alors de la racine, dans la même direction ;
    /// négatif, il la déplie, jusqu'à la tendre tout à fait) ; au
    /// delà de sa longueur, la chaîne s'étire jusqu'à `max_stretch`, puis
    /// pointe vers la cible. Plus près qu'au repos, elle se tasse d'abord
    /// (jusqu'à `give` de sa longueur) avant de plier : une patte d'appui
    /// reste droite, comme celle d'un vrai mouton.
    pub(crate) fn reach(
        &self,
        root: Point,
        target: Point,
        natural: f64,
        fold: f64,
        max_stretch: f64,
        give: f64,
    ) -> Bend {
        let (l1, l2) = (self.l1, self.l2);
        let to = target - root;
        let d = to.hypot();
        let straight = l1 + l2;
        let rest = self.rest_len;
        let stretch = if d > straight {
            (d / straight).min(max_stretch.max(1.0))
        } else if d < rest {
            (d / rest).max(1.0 - give.clamp(0.0, 0.5))
        } else {
            1.0
        };
        let d = (d / stretch).clamp((l1 - l2).abs() + 1e-6, straight);
        self.solve(to, d, l1, l2, natural, fold, stretch)
    }

    /// Comme [`Self::reach`], mais seul le premier segment s'étire (une
    /// encolure qui s'allonge sous une tête qui reste d'une pièce).
    pub(crate) fn reach_first(
        &self,
        root: Point,
        target: Point,
        natural: f64,
        max_stretch: f64,
    ) -> Bend {
        let (l1, l2) = (self.l1, self.l2);
        let to = target - root;
        let d = to.hypot();
        let stretch = ((d - l2) / l1).clamp(1.0, max_stretch.max(1.0));
        let l1 = l1 * stretch;
        let d = d.clamp((l1 - l2).abs() + 1e-6, l1 + l2);
        self.solve(to, d, l1, l2, natural, 0.0, stretch)
    }

    /// Les angles qui posent le bout à la distance `d` dans la direction `to`,
    /// les segments mesurant `l1` et `l2`.
    #[allow(clippy::too_many_arguments)]
    fn solve(
        &self,
        to: Vec2,
        d: f64,
        l1: f64,
        l2: f64,
        natural: f64,
        fold: f64,
        stretch: f64,
    ) -> Bend {
        let c = ((d * d - l1 * l1 - l2 * l2) / (2.0 * l1 * l2)).clamp(-1.0, 1.0);
        let side = self.side(natural);
        let bend = side * (c.acos() + fold).clamp(0.0, PI - 0.05);
        let offset = (l2 * bend.sin()).atan2(l1 + l2 * bend.cos());
        let first_dir = angle(to) - offset;
        Bend {
            first: wrap(first_dir - self.first_rest),
            second: wrap(bend - self.bend_rest),
            stretch,
        }
    }

    /// Où sont l'articulation et le bout une fois pliés, la racine en `root`.
    #[cfg(test)]
    pub(crate) fn place(&self, root: Point, bend: Bend) -> (Point, Point) {
        let joint = root + rotate(self.joint - self.root, bend.first) * bend.stretch;
        let tip = joint + rotate(self.tip - self.joint, bend.first + bend.second) * bend.stretch;
        (joint, tip)
    }
}

/// Le premier segment posé : déplacé jusqu'à `root`, tourné autour de sa
/// racine, étiré le long de lui-même.
pub(crate) fn first_segment(chain: &Chain, root: Point, bend: Bend) -> Affine {
    let turn = Affine::translate(root - chain.root) * rotate_about(chain.root, bend.first);
    if (bend.stretch - 1.0).abs() < 1e-12 {
        return turn;
    }
    // Étiré le long de l'axe de la chaîne (de la racine au bout, au repos).
    let (c, s) = chain.axis;
    let k = bend.stretch - 1.0;
    let m = Affine::new([
        1.0 + k * c * c,
        k * c * s,
        k * c * s,
        1.0 + k * s * s,
        0.0,
        0.0,
    ]);
    turn * about(chain.root, m)
}

/// Le second segment, dans le repère du premier : il tourne autour de
/// l'articulation.
pub(crate) fn second_segment(chain: &Chain, bend: Bend) -> Affine {
    rotate_about(chain.joint, bend.second)
}

// --------------------------------------------------------------- encolure

/// Ce que le moteur voulait de la tête d'une pièce (voir
/// [`crate::animate::pose`]) : sa rotation autour de la nuque et son
/// déplacement, sans le broutage ni le sommeil tête au sol (refaits ici).
#[derive(Clone, Copy, Debug)]
pub(crate) struct HeadIntent {
    pub angle: f64,
    pub shift: Vec2,
}

/// L'encolure au repos : sa base (devant le garrot), la nuque, le museau (ce
/// qui touche l'herbe), et la boîte de la tête.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NeckRest {
    pub base: Point,
    pub poll: Point,
    pub muzzle: Point,
    pub head: Rect,
}

/// Ce que font l'encolure et la tête : l'encolure autour de sa base (et
/// étirée le long d'elle-même), la tête autour de la nuque, d'une pièce
/// (elle ne s'étire jamais).
#[derive(Clone, Copy, Debug)]
pub(crate) struct NeckPose {
    pub neck: Affine,
    pub head: Affine,
    /// L'angle de la tête dans le dessin (pour garder les pupilles à
    /// l'horizontale).
    pub head_angle: f64,
}

/// Les angles de l'encolure (radians, positif : vers le bas). Aux aguets,
/// l'encolure se redresse ; quand il fonce, elle se tend, tête basse ; à
/// moitié endormi, elle s'affaisse un peu.
const NECK_ALERT: f64 = -0.24;
const NECK_CHARGE: f64 = 0.3;
const NECK_DOZE: f64 = 0.14;
/// En broutant, le museau va au sol à tant de fois la longueur de la tête
/// (de la nuque au museau) devant les sabots des antérieurs, et la tête
/// mordille.
pub(crate) const GRAZE_REACH: f64 = 0.85;
/// Jusqu'où l'encolure s'étire pour poser le museau au sol.
const NECK_STRETCH: f64 = 1.12;
/// Endormi la tête au sol : l'encolure s'allonge vers l'avant, la tête
/// presque à plat.
const SLEEP_STRETCH: f64 = 1.08;
const SLEEP_HEAD: f64 = 0.12;

/// Ce qui mène l'encolure à cette image.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NeckDrive {
    /// La base de l'encolure, portée par le corps.
    pub base: Point,
    pub intent: HeadIntent,
    /// Aux aguets (0 à 1), il fonce tête basse (0 à 1), il somnole (0 à 1).
    pub alert: f64,
    pub charge: f64,
    pub doze: f64,
    /// Il broute (0 à 1), ses coups de dents (-1 à 1), et les sabots des
    /// antérieurs (le museau va au sol devant eux).
    pub graze: f64,
    pub nibble: f64,
    pub graze_at: Point,
    /// Endormi la tête au sol (0 à 1).
    pub sleep_low: f64,
    /// La hauteur du sol, un centième de la hauteur de l'animal.
    pub ground: f64,
    pub u: f64,
    /// La tête qui tourne d'un côté à l'autre, vue de profil (voir
    /// [`Gesturing::head_squash`]).
    pub squash: f64,
}

impl NeckRest {
    fn chain(&self) -> Chain {
        Chain::new(self.base, self.poll, self.muzzle)
    }

    /// Le pose de l'encolure et de la tête.
    pub(crate) fn pose(&self, d: &NeckDrive) -> NeckPose {
        let neck_len = (self.poll - self.base).hypot().max(1e-6);
        let rest_dir = angle(self.poll - self.base);
        // Ce que voulait le moteur : la nuque déplacée de `shift`. L'encolure
        // tourne autour de sa base pour l'y porter, et s'allonge un peu
        // s'il le faut ; la tête garde l'angle voulu.
        let wanted = self.poll + d.intent.shift;
        let to = wanted - d.base;
        let mut first = wrap(angle(to) - rest_dir);
        let mut stretch = (to.hypot() / neck_len).clamp(0.94, 1.1);
        first += NECK_ALERT * d.alert + NECK_CHARGE * d.charge + NECK_DOZE * d.doze;
        stretch += 0.04 * d.charge;
        let mut head = d.intent.angle + 0.06 * d.alert;
        let mut bend = Bend {
            first,
            second: head - first,
            stretch,
        };
        // Broute : le museau au sol, devant les antérieurs.
        if d.graze > 0.0 {
            let chain = self.chain();
            let mut target = d.graze_at + Vec2::new(GRAZE_REACH * self.head_length(), 0.0);
            target.y -= 0.6 * d.u * d.nibble.max(0.0);
            // Une encolure trop courte pour aller si loin : le museau se pose
            // plus près des antérieurs, mais il touche l'herbe.
            let reach = 0.99 * (NECK_STRETCH * chain.l1 + chain.l2);
            let dy = target.y - d.base.y;
            if dy < reach {
                target.x = target.x.min(d.base.x + (reach * reach - dy * dy).sqrt());
            }
            let mut ik = chain.reach_first(d.base, target, 1.0, NECK_STRETCH);
            ik.second += 0.05 * d.nibble;
            bend = bend.lerp(ik, d.graze);
        }
        // Endormi la tête au sol : la tête presque à plat, l'encolure tendue
        // vers l'avant juste assez pour que le menton touche.
        if d.sleep_low > 0.0 {
            let lowest = lowest_point(&rotate_about(self.poll, SLEEP_HEAD), self.head);
            let drop = lowest - self.poll.y;
            let want = d.ground - d.u - drop - d.base.y;
            let mut stretch = SLEEP_STRETCH;
            if want > stretch * neck_len {
                stretch = (want / neck_len).min(1.15);
            }
            let q = (want / (stretch * neck_len)).clamp(-1.0, 1.0);
            let first = wrap(q.asin() - rest_dir);
            let low = Bend {
                first,
                second: SLEEP_HEAD - first,
                stretch,
            };
            bend = bend.lerp(low, d.sleep_low);
        }
        head = bend.first + bend.second;
        let neck = Affine::translate(d.base - self.base)
            * rotate_about(self.base, bend.first)
            * stretch_along(self.base, rest_dir, bend.stretch);
        let poll = d.base + rotate(self.poll - self.base, bend.first) * bend.stretch;
        let head_t = Affine::translate(poll - self.poll)
            * rotate_about(self.poll, head)
            * crate::animate::scale_about(self.head.center(), 1.0 - d.squash, 1.0);
        NeckPose {
            neck,
            head: head_t,
            head_angle: head,
        }
    }

    /// De la nuque au museau.
    pub(crate) fn head_length(&self) -> f64 {
        (self.muzzle - self.poll).hypot()
    }
}

// --------------------------------------------------------------- pattes

/// Une patte en deux segments, au repos : l'épaule ou la hanche, le carpe
/// ou le jarret, le bas du sabot.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LegRest {
    pub chain: Chain,
    pub front: bool,
    pub near: bool,
    /// La demi-largeur du canon (le genou plié se pose sur le sol).
    pub half_width: f64,
}

/// Le côté où plie l'articulation : le canon d'un antérieur part en arrière
/// (le carpe pointe vers l'avant), celui d'un postérieur vers l'avant (le
/// jarret pointe vers l'arrière).
fn natural(front: bool) -> f64 {
    if front { 1.0 } else { -1.0 }
}

/// De combien plient les articulations en l'air (radians) : pattes
/// repliées en montant. En vol, au pas, le carpe se plie un peu plus que
/// ce que demande la levée du sabot (le canon part vers l'arrière).
const TUCK_FOLD: (f64, f64) = (1.7, 1.3);
const SWING_FOLD: (f64, f64) = (0.25, 0.0);
/// Jusqu'où une patte s'étire (au bout de l'appui, tenue en l'air), et de
/// combien elle se tasse à l'appui avant de plier.
const LEG_STRETCH: f64 = 1.25;
const LEG_GIVE: f64 = 0.07;

/// Ce qui mène les pattes à cette image : les grandeurs du moteur.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LegDrive {
    pub gait: Gait,
    pub phase: f64,
    pub duty: f64,
    pub stride: f64,
    pub u: f64,
    pub ground: f64,
    pub air: f64,
    pub tuck: f64,
    pub reach: f64,
    pub loose: f64,
    pub dangle: f64,
    pub swing: f64,
    pub splay: f64,
    pub front_down: f64,
    pub back_down: f64,
    pub act: Gesturing,
    /// L'inclinaison de tout l'animal (l'élan, voir
    /// [`crate::animate::root_transform`]), défaite pour les sabots au sol :
    /// posés, ils restent sur le sol de l'écran, ni dessous ni au-dessus.
    pub untilt: Affine,
}

impl LegDrive {
    /// Où vont la patte et son bas : le premier segment, libre dans le
    /// dessin, la hanche (ou l'épaule) posée en `hip` ; le second, dans le
    /// repère du premier.
    pub(crate) fn pose(&self, leg: &LegRest, hip: Point) -> (Affine, Affine) {
        let bend = self.bend(leg, hip);
        (
            first_segment(&leg.chain, hip, bend),
            second_segment(&leg.chain, bend),
        )
    }

    pub(crate) fn bend(&self, leg: &LegRest, hip: Point) -> Bend {
        let LegRest {
            chain, front, near, ..
        } = *leg;
        let u = self.u;
        let moving = self.gait.moving();
        let leg_len = (self.ground - chain.root.y).max(u * 8.0);
        // Le chemin du sabot : posé, il recule exactement de ce que le corps
        // avance pendant l'appui (il ne glisse pas) ; levé, il revient vers
        // l'avant.
        let cycle = self.phase + crate::animate::leg_phase(front, near, &self.gait);
        let (forward, raised, flight) = hoof_path(cycle, self.duty, front);
        let half = (self.stride * self.duty * 0.5).min(0.93 * leg_len);
        let lift = raised * self.gait.lift(u);
        let rest = chain.tip;
        let mut target = self.untilt * Point::new(rest.x + moving * half * forward, rest.y - lift);
        // Ce que le moteur faisait de la patte d'une pièce : la tourner
        // autour de la hanche (en l'air, tenue, à l'atterrissage, les gestes).
        let mut turn = if front {
            -0.15 * self.air * self.loose + self.tuck - 0.3 * self.reach
        } else {
            0.15 * self.air * self.loose + 0.55 * self.tuck - 0.12 * self.reach
        };
        turn += self.dangle * (if front { -0.1 } else { 0.12 } + self.swing * 0.4);
        // À l'atterrissage, les sabots s'écartent un peu en glissant sur le
        // sol ; ce sont surtout les genoux qui plient.
        let splay = leg_len * (0.4 * self.splay).sin();
        target.x += if front { splay } else { -splay };
        let slot = leg_slot(front, near);
        let (gesture_angle, gesture_lift) = self.act.legs[slot];
        turn += gesture_angle;
        if turn != 0.0 {
            target = hip + rotate(target - hip, turn);
        }
        target.y -= gesture_lift * u;
        // Tenue ou tendue vers le sol, la patte pend de tout son long.
        let hang = 1.0 + 0.07 * self.dangle + 0.04 * self.air;
        target = hip + (target - hip) * hang;
        let (tuck, swing) = if front {
            (TUCK_FOLD.0, SWING_FOLD.0)
        } else {
            (TUCK_FOLD.1, SWING_FOLD.1)
        };
        let fold =
            tuck * self.tuck + swing * moving * flight * (1.0 - self.air) + self.act.folds[slot];
        // À l'appui, la patte reste droite (elle se tasse un rien) ; dès que
        // le sabot quitte le sol, l'articulation plie.
        let give =
            LEG_GIVE * (1.0 - (4.0 * flight).min(1.0)) * (1.0 - self.air) * (1.0 - self.dangle);
        let mut bend = chain.reach(hip, target, natural(front), fold, LEG_STRETCH, give);
        // Couché : les antérieurs repliés sous le poitrail (le carpe plié à
        // fond, le canon sous le corps), les postérieurs le long du ventre
        // (le jarret derrière, le canon vers l'avant). Il plie l'avant
        // d'abord, et se relève l'arrière d'abord.
        let down = if front {
            self.front_down
        } else {
            self.back_down
        };
        if down > 0.0 {
            bend = bend.lerp(self.lying(leg, hip), down);
        }
        bend
    }

    /// La patte repliée sous l'animal couché.
    fn lying(&self, leg: &LegRest, hip: Point) -> Bend {
        let chain = &leg.chain;
        let l1 = chain.l1;
        // L'articulation se pose au sol (ou aussi bas que le segment le
        // permet) ; le canon s'allonge à plat, vers l'arrière (antérieur) ou
        // vers l'avant (postérieur).
        let knee_y = self.ground - leg.half_width;
        let drop = ((knee_y - hip.y) / l1).clamp(-1.0, 1.0);
        let from_down = drop.acos();
        let rest_first = chain.first_rest;
        let (first_dir, cannon_dir) = if leg.front {
            // Le bras vers l'avant et le bas, le canon replié dessous, vers l'arrière.
            (FRAC_PI_2 - from_down.min(1.25), PI - 0.08)
        } else {
            // La cuisse vers l'arrière, le canon vers l'avant, le long du ventre.
            (FRAC_PI_2 + from_down.min(1.3), 0.06)
        };
        let first = wrap(first_dir - rest_first);
        let rest_second = angle(chain.tip - chain.joint);
        // La rotation du second segment, en plus de celle du premier.
        let second = wrap(cannon_dir - rest_second - first);
        Bend {
            first,
            second,
            stretch: 1.0,
        }
    }
}

/// Le chemin d'un sabot dans son cycle (`cycle` en radians), comme
/// [`stepping`] : vers l'avant (1) ou l'arrière (-1) de sa place, sa levée
/// (0 à 1), et la part du vol (0 à l'appui, jusqu'à 1 au milieu du vol).
/// À l'appui, il recule régulièrement : posé, il ne glisse pas.
pub(crate) fn hoof_path(cycle: f64, duty: f64, front: bool) -> (f64, f64, f64) {
    let t = cycle.rem_euclid(2.0 * PI) / (2.0 * PI);
    if t < duty {
        (1.0 - 2.0 * t / duty, 0.0, 0.0)
    } else {
        let s = (t - duty) / (1.0 - duty);
        let e = ease(0.0, 1.0, s);
        let (_, raised, _) = stepping(cycle, duty, 1.0, front);
        (2.0 * e - 1.0, raised, (PI * s).sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leg(front: bool) -> Chain {
        if front {
            Chain::new(
                Point::new(100.0, 88.0),
                Point::new(101.0, 100.0),
                Point::new(100.5, 112.0),
            )
        } else {
            Chain::new(
                Point::new(49.0, 88.0),
                Point::new(46.0, 100.0),
                Point::new(49.0, 112.0),
            )
        }
    }

    #[test]
    fn a_two_bone_chain_reaches_its_target_and_bends_the_natural_way() {
        for front in [true, false] {
            let chain = leg(front);
            // Au repos, rien ne bouge.
            let rest = chain.reach(chain.root, chain.tip, natural(front), 0.0, 1.1, 0.07);
            assert!(
                rest.first.abs() < 1e-9 && rest.second.abs() < 1e-9,
                "{rest:?}"
            );
            // Le sabot levé de 5 et avancé de 3 : il y est, et le genou plie
            // du bon côté (le carpe en avant de la ligne, le jarret en arrière).
            let target = chain.tip + Vec2::new(3.0, -5.0);
            let bend = chain.reach(chain.root, target, natural(front), 0.0, 1.1, 0.0);
            let (knee, tip) = chain.place(chain.root, bend);
            assert!((tip - target).hypot() < 1e-6, "{tip:?}");
            let line = target - chain.root;
            let side = line.cross(knee - chain.root);
            if front {
                assert!(side < 0.0, "le carpe pointe vers l'avant");
            } else {
                assert!(side > 0.0, "le jarret pointe vers l'arrière");
            }
            // Trop loin : la chaîne s'étire, puis pointe vers la cible.
            let far = chain.root + (chain.tip - chain.root) * 1.5;
            let bend = chain.reach(chain.root, far, natural(front), 0.0, 1.1, 0.0);
            assert!((bend.stretch - 1.1).abs() < 1e-9);
            // Un peu plus près qu'au repos, à l'appui : elle se tasse sans plier.
            let near = chain.root + (chain.tip - chain.root) * 0.97;
            let bend = chain.reach(chain.root, near, natural(front), 0.0, 1.1, 0.07);
            assert!(bend.second.abs() < 1e-9 && (bend.stretch - 0.97).abs() < 1e-9);
        }
    }

    #[test]
    fn a_planted_hoof_moves_back_at_the_speed_of_the_body() {
        // À l'appui, le sabot recule d'autant que la phase avance : comme
        // la phase suit la distance parcourue, il reste où il est posé.
        let duty = 0.64;
        let a = hoof_path(0.1, duty, true).0;
        let b = hoof_path(0.3, duty, true).0;
        let c = hoof_path(0.5, duty, true).0;
        assert!(((a - b) - (b - c)).abs() < 1e-9);
        assert!(hoof_path(0.2, duty, false).1 == 0.0);
        // En vol, il est levé ; au bout du vol, il retrouve l'avant.
        let mid = hoof_path(2.0 * PI * (duty + (1.0 - duty) / 2.0), duty, false);
        assert!(mid.1 > 0.9 && mid.2 > 0.9);
        let end = hoof_path(2.0 * PI * 0.9999, duty, true);
        assert!((end.0 - 1.0).abs() < 1e-3);
    }
}
